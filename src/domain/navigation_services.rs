use std::sync::Arc;

use crate::domain::aggregates::location::Location;
use crate::domain::aggregates::player_state::PlayerState;
use crate::port::repository::{LocationRepository, PassageRepository};

pub struct NavigationService {
    location_repository: Arc<dyn LocationRepository>,
    passage_repository: Arc<dyn PassageRepository>,
}

impl NavigationService {
    pub fn new(location_repository: Arc<dyn LocationRepository>, passage_repository: Arc<dyn PassageRepository>) -> Self {
        NavigationService { location_repository, passage_repository }
    }

    // Example method to navigate based on direction
    pub fn navigate(&self, player_state: PlayerState, direction: String) -> Result<(Location, String), String> {
        if let Some(passage) = self.passage_repository.find_passage_by_direction_and_location(player_state.current_location_id, &*direction) {
            if let Some(target_location) = self.location_repository.get_location_by_id(passage.to_location_id) {
                let narration = format!("You {} and reach {}.", passage.narration, target_location.title);
                Ok((target_location, narration))
            } else {
                Err("Target location not found.".to_string())
            }
        } else {
            Err("No passage found in that direction.".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::*;
    use mockall::predicate::*;

    use super::*;
    use super::super::aggregates::passage::{PassageBuilder, Passage};

    // Mocking the LocationRepository
    mock! {
        LocationRepository {}

        trait LocationRepository{
            fn get_location_by_id(&self, id: i32) -> Option<Location>;
            fn get_all_locations(&self) -> Vec<Location>;
            fn add_location(&self, location: Location) -> Result<(), String>;
        }
    }

    // Mocking the PassageRepository
    mock! {
        PassageRepository {}

         trait PassageRepository {
            fn find_passage_by_direction_and_location(&self, location_id: i32, direction: &str) -> Option<Passage>;
            fn get_passage_by_id(&self, id: i32) -> Option<Passage>;
            fn get_passages_for_location(&self, location_id: i32) -> Vec<Passage>;
            fn add_passage(&self, passage: Passage) -> Result<(), String>;
        }
    }


    #[test]
    fn navigate_to_passage_success() {
        let mut mock_passage_repo = MockPassageRepository::new();
        let mock_location_repo = MockLocationRepository::new(); // Not used in this test, but required for NavigationService instantiation

        // Setting up the mock to expect a call with specific parameters and to return a specific result
        mock_passage_repo.expect_find_passage_by_direction_and_location()
            .with(eq(1), eq("north"))
            .times(1) // Expect it to be called exactly once
            .returning(|_, _| Some(PassageBuilder::default()
                .id(1)
                .from_location_id(1)
                .to_location_id(2)
                .description("A passage".into())
                .direction("north".into())
                .narration("You go north".into())
                .build()
                .unwrap()));

        let navigation_service = NavigationService::new(Arc::new(mock_location_repo), Arc::new(mock_passage_repo));

        let player_state_instance = PlayerState::new(1,1);

        let result = navigation_service.navigate(player_state_instance, "north".into());
        assert!(result.is_ok());
        let (resultloc, resultstr) = result.unwrap();

        println!("location is {}", resultloc.id);
        println!("string is {}", resultstr);

//        assert_eq!(resultloc.id, 2);
//        assert_eq!(resultstr, "You go north");
    }

    // Add more tests here, such as `navigate_to_passage_failure` to test the behavior when no passage is found.
}

