use std::sync::Arc;

use domain::aggregates::location::Location;
use domain::aggregates::player_state::PlayerState;
use port::repository::{LocationRepository, PassageRepository};

// Wrap Service in a Trait to allow mocking for tests
// Send + Sync traits for threads safely
pub trait NavigationServiceTrait: Send + Sync {
fn navigate(&self, player_state: PlayerState, direction: String) -> Result<(Location, String), String>;
}

pub struct NavigationService {
    location_repository: Arc<dyn LocationRepository>,
    passage_repository: Arc<dyn PassageRepository>,
}

impl NavigationService {
    pub fn new(location_repository: Arc<dyn LocationRepository>, passage_repository: Arc<dyn PassageRepository>) -> Self {
        NavigationService { location_repository, passage_repository }
    }
}


impl NavigationServiceTrait for NavigationService {
    // Example method to navigate based on direction
    fn navigate(&self, player_state: PlayerState, direction: String) -> Result<(Location, String), String> {
        if let Some(passage) = self.passage_repository.find_passage_by_direction_and_location(player_state.current_location_id(), &*direction) {
            if let Some(target_location) = self.location_repository.get_location_by_id(passage.get_to_location()) {
                let narration = format!("{} and reach {}.", passage.get_narration_reference(), target_location.get_title_reference());
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
    use domain::aggregates::location::LocationBuilder;

    use super::*;
    use domain::aggregates::passage::PassageBuilder;
    use domain::aggregates::passage::Passage;

    // Mocking the LocationRepository
    mock! {
        LocationRepository {}

        impl LocationRepository for LocationRepository{
            fn get_location_by_id(&self, id: i32) -> Option<Location>;
            fn get_all_locations(&self) -> Vec<Location>;
            fn add_location(&self, location: Location) -> Result<(), String>;
        }
    }

    // Mocking the PassageRepository
    mock! {
        PassageRepository {}

         impl PassageRepository for PassageRepository {
            fn find_passage_by_direction_and_location(&self, location_id: i32, direction: &str) -> Option<Passage>;
            fn get_passage_by_id(&self, id: i32) -> Option<Passage>;
            fn get_passages_for_location(&self, location_id: i32) -> Vec<Passage>;
            fn add_passage(&self, passage: Passage) -> Result<(), String>;
        }
    }

    #[test]
    fn navigate_to_passage_success() {
        let mut mock_passage_repo = MockPassageRepository::new();
        let mut mock_location_repo = MockLocationRepository::new();

        mock_passage_repo.expect_find_passage_by_direction_and_location()
            .with(eq(1), eq("north"))
            .times(1)
            .returning(|_, _| Some(PassageBuilder::default()
                .aggregate_id(1)
                .from_location_id(1)
                .to_location_id(2)
                .description("A passage".into())
                .direction("north".into())
                .narration("You go north".into())
                .build()
                .unwrap()));

        mock_location_repo.expect_get_location_by_id()
            .with(eq(2)) // Assuming `to_location_id` is 2 as set in the PassageBuilder
            .times(1)
            .returning(|_| Some(LocationBuilder::default()
                .aggregate_id(2)
                .title("Target Location".into())
                .description("abc".into())
                .image_url(None)
                .build()
                .unwrap()));

        let navigation_service = NavigationService::new(Arc::new(mock_location_repo), Arc::new(mock_passage_repo));

        let player_state_instance = PlayerState::new(1, 1);

        let result = navigation_service.navigate(player_state_instance, "north".into());
        assert!(result.is_ok());
        let (location, narration) = result.unwrap();

        assert_eq!(location.get_aggregate_id(), 2);
        assert_eq!(narration, "You go north and reach Target Location.");
    }
}
