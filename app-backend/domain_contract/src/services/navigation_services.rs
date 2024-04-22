use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use mockall::PredicateBoxExt;
use crosscutting::error_management::error::Error;
use crosscutting::error_management::error_kind::ErrorKind;

use domain_pure::model::location::Location;
use domain_pure::model::player_state::PlayerState;

use crate::contracts::location_query::navigation::LocationQueries;
use crate::contracts::passage_query::navigation::PassageQueries;

// Wrap Service in a Trait to allow mocking for tests
// Send + Sync traits for threads safely

pub trait NavigationServiceTrait: Send + Sync {
    fn navigate(
        &self,
        player_state: PlayerState,
        direction: String,
    ) -> Pin<Box<dyn Future<Output = Result<(Location, String), Error>> + Send>>;
}

pub struct NavigationService {
    passage_query: Arc<dyn PassageQueries>,
    location_query: Arc<dyn LocationQueries>,
}

impl NavigationService {
    pub fn new(
        location_query: Arc<dyn LocationQueries>,
        passage_query: Arc<dyn PassageQueries>,
    ) -> Self {
        NavigationService {
            location_query,
            passage_query,
        }
    }
}

impl NavigationServiceTrait for NavigationService {
    fn navigate(
        &self,
        player_state: PlayerState,
        direction: String,
    ) -> Pin<Box<dyn Future<Output = Result<(Location, String), Error>> + Send>> {
        let passage_query_clone = self.passage_query.clone();
        let location_query_clone = self.location_query.clone();

        async move {
            let passage_result = passage_query_clone
                .find_passage_by_location_and_direction(player_state.current_location_id(), &direction)
                .await;

            match passage_result {
                Ok(Some(passage)) => {
                    let location_result = location_query_clone
                        .get_location_by_aggregate_id(passage.get_to_location())
                        .await;

                    match location_result {
                        Ok(Some(target_location)) => {
                            let narration = format!("{} and reach {}.", passage.narration(), target_location.title());
                            Ok((target_location, narration))
                        },
                        Ok(None) => Err(Error::new("No passage found in that direction", ErrorKind::Functional)),
                        Err(e) => Err(e),
                    }
                },
                Ok(None) => Err(Error::new("No passage found in that direction", ErrorKind::Functional)),
                Err(e) => Err(e),
            }
        }
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::mock;
    use mockall::predicate::*;

    use domain_pure::model::location::{Location, LocationBuilder};
    use domain_pure::model::passage::{Passage, PassageBuilder};
    use domain_pure::model::player_state::PlayerStateBuilder;

    use super::*;

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

    #[tokio::test]
    async fn navigate_to_passage_success() {
        let mut mock_passage_query = MockPassageQueries::new();
        let mut mock_location_query = MockLocationQueries::new();

        mock_location_query.expect_get_location_by_aggregate_id()
            .with(eq(2_i32))
            .times(1)
            .returning(|_| Some(LocationBuilder::default()
                .aggregate_id(2)
                .title("Target Location".into())
                .description("Description of Target Location".into())
                .build().unwrap()));

        mock_passage_query.expect_find_passage_by_location_and_direction()
            .with(eq(1), eq("north"))
            .times(1)
            .returning(|_, _| Some(PassageBuilder::default()
                .aggregate_id(2)
                .from_location_id(1)
                .to_location_id(2)
                .description("Description of passage".into())
                .direction("north".into())
                .narration("You go north".into())
                .build().unwrap()));

        let navigation_service = NavigationService::new(Arc::new(mock_location_query), Arc::new(mock_passage_query));
        let player_state_instance = PlayerStateBuilder::default().player_id(1).current_location_id(1).build().unwrap();
        let result = navigation_service.navigate(player_state_instance, "north".into()).await;

        assert!(result.is_ok());
        let (location, narration) = result.unwrap();
        assert_eq!(location.aggregate_id(), 2);
        assert_eq!(narration, "You go north and reach Target Location.");
    }
}
