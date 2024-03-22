use std::sync::Arc;


use crate::domain::navigation_services::NavigationServiceTrait;
use crate::port::context::RequestContext;
use crate::port::dto::{MovePlayerCommand, MovePlayerResult};
use crate::port::repository::{LocationRepository, PassageRepository, PlayerStateRepository};

#[allow(dead_code)] // unused repositories will be used at a later point
pub struct MovePlayerUseCase {
    location_repository: Arc<dyn LocationRepository>,
    passage_repository: Arc<dyn PassageRepository>,
    player_state_repository: Arc<dyn PlayerStateRepository>,
    navigation_service: Arc<dyn NavigationServiceTrait>,
}

impl MovePlayerUseCase {
    pub fn new(
        location_repository: Arc<dyn LocationRepository>,
        passage_repository: Arc<dyn PassageRepository>,
        player_state_repository: Arc<dyn PlayerStateRepository>,
        navigation_service: Arc<dyn NavigationServiceTrait>,
    ) -> Self {
        Self { location_repository, passage_repository, player_state_repository, navigation_service }
    }

    pub fn execute(&self, input: MovePlayerCommand, context: RequestContext) -> Result<MovePlayerResult, String> {
        if let Some(player_id) = context.player_id {
            let mut player_state = match self.player_state_repository.find_by_id(player_id) {
                Some(state) => state,
                None => return Err("Player state not found".to_string()),
            };

            let (new_location, narration) = self.navigation_service.navigate(player_state.clone(), input.direction)?;

            // update player state

            player_state.set_current_location_id(new_location.id);
            self.player_state_repository.save(player_state);

            Ok(MovePlayerResult {
                player_location: new_location.id,
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

    use crate::domain::aggregates::location::Location;
    use crate::domain::aggregates::passage::Passage;
    use crate::domain::aggregates::player_state::PlayerState;
    use crate::domain::navigation_services::NavigationServiceTrait;

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
            fn get_location_by_id(&self, id: i32) -> Option<Location>;
            fn get_all_locations(&self) -> Vec<Location>;
            fn add_location(&self, location: Location) -> Result<(), String>;
        }
    }

    mock! {
        PassageRepository {}

         impl  PassageRepository for PassageRepository {
            fn find_passage_by_direction_and_location(&self, location_id: i32, direction: &str) -> Option<Passage>;
            fn get_passage_by_id(&self, id: i32) -> Option<Passage>;
            fn get_passages_for_location(&self, location_id: i32) -> Vec<Passage>;
            fn add_passage(&self, passage: Passage) -> Result<(), String>;
        }
    }

    mock! {
        PlayerStateRepository {}

        impl PlayerStateRepository for PlayerStateRepository  {
             fn find_by_id(&self, id: i32) -> Option<PlayerState>;
             fn save(&self, player_state: PlayerState);
        }
    }

    #[test]
    fn test_move_player_success() {
        let player_state_instance = PlayerState::new(1, 1);
        let direction = "north";

        let mut mock_passage_repo = MockPassageRepository::new();
        let mut mock_location_repo = MockLocationRepository::new();
        let mut mock_player_state_repo = MockPlayerStateRepository::new();
        let mut mock_navigation_service = MockNavigationService::new();


        // If NavigationService is used, setup expectations for navigating
        mock_navigation_service.expect_navigate()
            .with(eq(PlayerState::new(1,1)), eq(String::from("north")))
            .times(1)
            .returning(|_, _| Ok((Location {
                id: 2,
                title: "Destination".into(),
                description: "You've moved north.".into(),
                image_url: None,
            }, "You move north.".into())));

        mock_player_state_repo.expect_find_by_id()
            .with(eq(1)) // Expect it to be called with an ID of 1
            .times(1)    // Expect it to be called exactly once
            .returning(|_| Some(PlayerState::new(1,1)));

        mock_player_state_repo.expect_save()
            .withf(|ps| ps.player_id == 1) // Ensure the `PlayerState` has `id == 1`
            .times(1)
            .returning(|_| ());


        // Instantiate the MovePlayerUseCase with the mocked dependencies
        let use_case = MovePlayerUseCase::new(Arc::new(mock_location_repo), Arc::new(mock_passage_repo),Arc::new((mock_player_state_repo)), Arc::new(mock_navigation_service));

        // Execute the use case and assert the expected outcome

// Assume `command` and `context` are already properly initialized
    let command = MovePlayerCommand { direction: "north".to_string() };
    let context = RequestContext { /* fields as necessary */ player_id: Some(1) };

        let result = use_case.execute(command, context);

// It's a good practice to assert the result is Ok before unwrapping
        assert!(result.is_ok());

// Unwrap the result after checking it is Ok to extract the MovePlayerResult
        let move_result = result.unwrap();

// Now, assert the values within MovePlayerResult
        assert_eq!(move_result.player_location, 2); // Assuming '2' is the expected location ID
        assert_eq!(move_result.narration, "You move north.");
    }
}
