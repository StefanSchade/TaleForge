use std::sync::Arc;


use domain_contract::services::navigation_services::{NavigationService, NavigationServiceTrait};
use port::context::RequestContext;
use port::domain_stories::move_player::{MovePlayerCommand, MovePlayerResult, MovePlayerDomainStory};

use port::repositories::location_repository::LocationRepository;
use port::repositories::passage_repository::PassageRepository;
use port::repositories::player_state_repository::PlayerStateRepository;
use crate::dto_domain_mapping::player_state_mapper::player_state_map_dto_to_domain;


#[allow(dead_code)] // unused repositories will be used at a later point
#[derive(Clone)]
pub struct MovePlayerDomainStoryImpl {
    location_repository: Arc<dyn LocationRepository>,
    passage_repository: Arc<dyn PassageRepository>,
    player_state_repository: Arc<dyn PlayerStateRepository>,
    navigation_service: Arc<dyn NavigationServiceTrait>,
}

impl MovePlayerDomainStoryImpl {
    pub fn new(
        location_repository: Arc<dyn LocationRepository>,
        passage_repository: Arc<dyn PassageRepository>,
        player_state_repository: Arc<dyn PlayerStateRepository>,
        navigation_service: Arc<dyn NavigationServiceTrait>
    ) -> Self {
        Self {
            location_repository,
            passage_repository,
            player_state_repository,
            navigation_service,
        }
    }
}

impl MovePlayerDomainStory for MovePlayerDomainStoryImpl {

    fn execute(&self, context: RequestContext, input: MovePlayerCommand) -> Result<MovePlayerResult, String> {
        if let Some(player_id) = context.player_id {
            let mut player_state = match self.player_state_repository.find_by_id(player_id) {
                Some(state) => state,
                None => return Err("Player state not found".to_string()),
            };

            let (new_location, narration) = self.navigation_service.navigate(player_state_map_dto_to_domain(&player_state), input.direction)?;

            player_state.current_location_id = new_location.aggregate_id();
            self.player_state_repository.save(player_state);

            Ok(MovePlayerResult {
                player_location: new_location.aggregate_id(),
                narration: narration,
            })
        } else {
            Err("Player ID not found in context".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;
    use mockall::{mock, predicate::*};

    use domain_pure::model::location::{Location, LocationBuilder};
    use domain_pure::model::player_state::{PlayerState, PlayerStateBuilder};
    use domain_contract::services::navigation_services::NavigationServiceTrait;
    use port::dto::location_dto::LocationDTO;
    use port::dto::passage_dto::PassageDTO;
    use port::dto::player_state_dto::PlayerStateDTO;

    use super::*;

    mock! {
        NavigationService {}

            impl NavigationServiceTrait for NavigationService {
                fn navigate(&self, player_state: PlayerState, direction: String) -> Result<(Location, String), String>;
            }
        }

    mock! {
        LocationRepository {}

        impl LocationRepository for LocationRepository  {
            fn get_location_by_id(&self, id: i32) -> Option<LocationDTO>;
            fn get_all_locations(&self) -> Vec<LocationDTO>;
            fn add_location(&self, location: LocationDTO) -> Result<(), String>;
        }
    }

    mock! {
        PassageRepository {}

         impl  PassageRepository for PassageRepository {
            fn find_passage_by_location_and_direction(&self, location_id: i32, direction: &str) -> Option<PassageDTO>;
            fn get_passage_by_id(&self, id: i32) -> Option<PassageDTO>;
            fn get_passages_for_location(&self, location_id: i32) -> Vec<PassageDTO>;
            fn add_passage(&self, passage: PassageDTO) -> Result<(), String>;
            fn find_by_start_and_end_id(&self, from_location_id: i32, to_location_id:i32) -> Option<PassageDTO>;
        }
    }

    mock! {
        PlayerStateRepository {}

        impl PlayerStateRepository for PlayerStateRepository  {
             fn find_by_id(&self, id: i32) -> Option<PlayerStateDTO>;
             fn save(&self, player_state: PlayerStateDTO);
        }
    }

    #[test]
    fn test_move_player_success() {

        // magic numbers relevant to the test
        let expected_destination_location_id: i32 = 99;
        let expected_passage_narration_text: &str = "You've moved north.";
        let expected_direction_instruction: &str = "north";
        let expected_player_id: i32 = 42;

        let mock_passage_repo = MockPassageRepository::new();
        let mock_location_repo = MockLocationRepository::new();
        let mut mock_player_state_repo = MockPlayerStateRepository::new();
        let mut mock_navigation_service = MockNavigationService::new();

        // We need to pass `expected_location` to the closure `.returning()`, but since this closure might
        // be called multiple times we have to clone it first. Wrapping it into Arc reduces the overhead.
        // We thus guarantee immutability and just have to clone the reference, but not the struct itself.
        let expected_destination_location = Arc::new(LocationBuilder::default()
            .aggregate_id(expected_destination_location_id)
            .title("Destination - This could be anything".into())
            .description("Destinatinon Decription - This could be anything".into())
            .build()
            .unwrap());

        mock_navigation_service.expect_navigate()
            //.with(eq(PlayerState::new(expected_player_id,1)), eq(expected_direction_instruction.to_string()))
            .with(eq(
                PlayerStateBuilder::default()
                    .player_id(expected_player_id)
                    .current_location_id(1)
                    .build()
                    .unwrap()
            ), eq(expected_direction_instruction.to_string()))
            .times(1)
            .returning(move |_, _| {
                let loc = expected_destination_location.clone();
                Ok(((*loc).clone(), expected_passage_narration_text.to_string()))
            });

        mock_player_state_repo.expect_find_by_id()
            .with(eq(expected_player_id))
            .times(1)
            .returning(move |_| Some(

                PlayerStateDTO {
                    player_id: expected_player_id,
                    current_location_id: 1,
                }


                // PlayerStateBuilder::default()
                //     .player_id(expected_player_id)
                //     .current_location_id(1)
                //     .build()
                //     .unwrap()



            ));

        // `expected_player_id` is of type i32 and thus implements the `Copy` trait, implying that instead of
        // borrowing it, it will be copied by simply passing it without explicitly calling `.clone()`
        // However doing so results in an [E0373], so we apply what we would do with non-Copy data and move
        // ownership explicitly to the closure.
        mock_player_state_repo.expect_save()
            .withf(move |ps| ps.player_id == expected_player_id)
            .times(1)
            .returning(|_| ());

        let use_case =
            MovePlayerDomainStoryImpl::new(
                Arc::new(mock_location_repo),
                Arc::new(mock_passage_repo),
                Arc::new(mock_player_state_repo),
                Arc::new(mock_navigation_service));

        let command = MovePlayerCommand { direction: expected_direction_instruction.into() };
        let context = RequestContext { player_id: Some(expected_player_id) };

        let result = use_case.execute(context, command);

        assert!(result.is_ok());

        let move_player_result = result.unwrap();

        assert_eq!(move_player_result.player_location, expected_destination_location_id);
        assert_eq!(move_player_result.narration, expected_passage_narration_text.to_string());
    }
}
