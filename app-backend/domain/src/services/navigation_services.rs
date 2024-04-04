use std::sync::Arc;

use crate::model::location::{Location};
use crate::model::player_state::PlayerState;
use crate::contracts::location_queries::navigation::LocationQueries;
use crate::contracts::passage_queries::navigation::PassageQueries;

// Wrap Service in a Trait to allow mocking for tests
// Send + Sync traits for threads safely
pub trait NavigationServiceTrait: Send + Sync {
fn navigate(&self, player_state: PlayerState, direction: String) -> Result<(Location, String), String>;
}

pub struct NavigationService {
    passage_query: Arc<dyn PassageQueries>,
    location_query: Arc<dyn LocationQueries>,

}

impl NavigationService {
    pub fn new(location_query: Arc<dyn LocationQueries> , passage_query: Arc<dyn PassageQueries> ) -> Self {
        NavigationService { location_query: location_query , passage_query }
    }
}

impl NavigationServiceTrait for NavigationService {
    // Example method to navigate based on direction
    fn navigate(&self, player_state: PlayerState, direction: String) -> Result<(Location, String), String> {
        //if let Some(passage) = self.passage_repository.find_passage_by_direction_and_location(player_state.current_location_id(), &*direction) {
        if let Some(passage) = self.passage_query.find_passage_by_location_and_direction(player_state.current_location_id(), &*direction) {
            //if let Some(target_location) = self.location_repository.get_location_by_id(passage.get_to_location()) {
            if let Some(target_location) = self.location_query.get_location_by_aggregate_id(passage.get_to_location()) {
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
    use crate::model::location::{Location, LocationBuilder};

    use super::*;
    use crate::model::passage::{Passage, PassageBuilder};

     mock! {
        LocationQueries {}

        impl LocationQueries for LocationQueries {
             fn get_location_by_aggregate_id(&self, location_aggregate_id: i32) -> Option<Location>;
        }
    }
    mock! {
        PassageQueries {}

        impl PassageQueries for PassageQueries {
            fn find_passage_between_locations(&self, from_location_id: i32, to_location_id: i32) -> Option<Passage>;
            fn find_passage_by_location_and_direction(&self, location_id: i32, direction: &str) -> Option<Passage>;
        }
    }


    #[test]
    fn navigate_to_passage_success() {

        let mut mock_passage_query = MockPassageQueries::new();
        let mut mock_location_query = MockLocationQueries::new();

        mock_location_query.expect_get_location_by_aggregate_id().with(mockall::predicate::eq(2_i32))
            .times(1)
            .returning(|_| Some(LocationBuilder::default()
                .aggregate_id(2)
                .title("Target Location".into())
                .description("Description of Target Location".into())
                .build().unwrap()));

        mock_passage_query.expect_find_passage_by_location_and_direction()
            .with(eq(1), eq("north"))
            .times(1)
            .returning(|_,_| Some(PassageBuilder::default()
                .aggregate_id(2)
                .from_location_id(1)
                .to_location_id(2)
                .description("Description of passage".into())
                .direction("north".into())
                .narration("You go north".into())
                .build().unwrap()));

        let navigation_service = NavigationService::new(Arc::new(mock_location_query), Arc::new(mock_passage_query));

        let player_state_instance = PlayerState::new(1, 1);

        let result = navigation_service.navigate(player_state_instance, "north".into());
        assert!(result.is_ok());
        let (location, narration) = result.unwrap();

        assert_eq!(location.get_aggregate_id(), 2);
        assert_eq!(narration, "You go north and reach Target Location.");
    }
}
