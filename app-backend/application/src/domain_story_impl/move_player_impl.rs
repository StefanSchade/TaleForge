use std::sync::Arc;
use async_trait::async_trait;

use domain_contract::services::navigation_services::{NavigationService, NavigationServiceTrait};
use port::context::RequestContext;
use port::domain_stories::move_player::{MovePlayerCommand, MovePlayerDomainStory, MovePlayerResult};
use port::repositories::location_repository::LocationRepository;
use port::repositories::passage_repository::PassageRepository;
use port::repositories::player_state_repository::PlayerStateRepository;
use crate::contract_implementations::location_query_impl::LocationQueryImpl;
use crate::contract_implementations::passage_query_impl::PassageQueryImpl;

use crate::dto_domain_mapping::player_state_mapper::player_state_map_dto_to_domain;

#[allow(dead_code)]
#[derive(Clone)]
pub struct MovePlayerDomainStoryImpl {
    location_repository: Arc<dyn LocationRepository>, // unused for the moment
    passage_repository: Arc<dyn PassageRepository>,
    player_state_repository: Arc<dyn PlayerStateRepository>,
    navigation_service: Arc<dyn NavigationServiceTrait>,
}

impl MovePlayerDomainStoryImpl {
    pub fn new(
        location_repository: Arc<dyn LocationRepository>,
        passage_repository: Arc<dyn PassageRepository>,
        player_state_repository: Arc<dyn PlayerStateRepository>,
    ) -> Self {
        // Clone the Arc for use in the NavigationService
        let location_query = Arc::new(LocationQueryImpl::new(location_repository.clone()));
        let passage_query = Arc::new(PassageQueryImpl::new(passage_repository.clone()));

        Self {
            // Pass the original Arc to be stored in the struct
            location_repository,
            passage_repository,
            player_state_repository,
            // Use the clones for creating the NavigationService
            navigation_service: Arc::new(NavigationService::new(location_query, passage_query)),
        }
    }
}


#[async_trait]
impl MovePlayerDomainStory for MovePlayerDomainStoryImpl {
    async fn execute(&self, context: RequestContext, input: MovePlayerCommand) -> Result<MovePlayerResult, String> {
        if let Some(player_id) = context.player_id {
            let mut player_state = match self.player_state_repository.find_by_player_id(player_id) {
                Some(state) => state,
                None => return Err("Player state not found".to_string()),
            };

            let (new_location, narration) = self.navigation_service.navigate(player_state_map_dto_to_domain(&player_state), input.direction).await?;
            player_state.current_location_id = new_location.aggregate_id();
            self.player_state_repository.save(player_state);
            Ok(MovePlayerResult {
                player_location: new_location.aggregate_id(),
                narration,
            })
        } else {
            Err("Player ID not found in context".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::{mock, predicate::*};
    use mockall::predicate::eq;

    use domain_contract::services::navigation_services::NavigationServiceTrait;
    use domain_pure::model::location::{Location, LocationBuilder};
    use domain_pure::model::player_state::{PlayerState, PlayerStateBuilder};
    use port::dto::location_dto::LocationDTO;
    use port::dto::passage_dto::PassageDTO;
    use port::dto::player_state_dto::PlayerStateDTO;

    use super::*;

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
             fn find_by_player_id(&self, id: i32) -> Option<PlayerStateDTO>;
             fn save(&self, player_state: PlayerStateDTO);
        }
    }

    #[test]
    fn test_move_player_success() {
        let expected_destination_location_id: i32 = 99;
        let expected_passage_narration_text: &str = "You take the passage and reach the new location.";
        let expected_direction_instruction: &str = "north";
        let expected_player_id: i32 = 42;

        let mut mock_location_repo = MockLocationRepository::new();
        let mut mock_passage_repo = MockPassageRepository::new();
        let mut mock_player_state_repo = MockPlayerStateRepository::new();

        // We need to pass `expected_location` to the closure `.returning()`, but since this closure might
        // be called multiple times we have to clone it first. Wrapping it into Arc reduces the overhead.
        // We thus guarantee immutability and just have to clone the reference, but not the struct itself.
        let expected_destination_location = Arc::new(LocationBuilder::default()
            .aggregate_id(expected_destination_location_id)
            .title("Destination - This could be anything".into())
            .description("Destinatinon Decription - This could be anything".into())
            .build()
            .unwrap());

        mock_passage_repo.expect_find_passage_by_location_and_direction()
            .with(mockall::predicate::eq(1), mockall::predicate::eq("north"))
            .times(1)  // You can specify how many times you expect this call
            .returning(|_, _| Some(PassageDTO {
                id: 1,
                description: "north".to_string(),
                direction: "north".to_string(),
                narration: "You take the passage".to_string(),
                from_location_id: 1,
                to_location_id: 1
            }));

        mock_location_repo.expect_get_location_by_id()
            .with(eq(1))
            .times(1)
            .returning(|_| Some(LocationDTO{
                id:99,
                title: "the new location".to_string(),
                description: "destination".to_string(),
                image_url: None,
            }));

        mock_player_state_repo.expect_find_by_player_id()
            .with(eq(expected_player_id))
            .times(1)
            .returning(move |_| Some(
                PlayerStateDTO {
                    player_id: expected_player_id,
                    current_location_id: 1,
                }
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
                Arc::new(mock_player_state_repo));

        let command = MovePlayerCommand { direction: expected_direction_instruction.into() };
        let context = RequestContext { player_id: Some(expected_player_id) };

        let result = use_case.execute(context, command);

        assert!(result.is_ok());

        let move_player_result = result.unwrap();

        assert_eq!(move_player_result.player_location, expected_destination_location_id);
        assert_eq!(move_player_result.narration, expected_passage_narration_text.to_string());
    }
}
