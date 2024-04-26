use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use futures::FutureExt;

use crosscutting::error_management::error::Error;
use crosscutting::error_management::standard_errors::NO_ENTRY_FOUND;
use domain_pure::model::location::Location;
use domain_pure::model::player_state::PlayerState;

use crate::contracts::location_query::navigation::LocationQueries;
use crate::contracts::passage_query::PassageQueries;

// Wrap Service in a Trait to allow mocking for tests
// Send + Sync traits for threads safely

pub trait NavigationServiceTrait: Send + Sync + Debug {
    fn navigate(
        &self,
        game_id: u64,
        player_state: PlayerState,
        direction: String,
    ) -> Pin<Box<dyn Future<Output=Result<(Location, String), Error>> + Send>>;
}

#[derive(Clone, Debug)]
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
        game_id: u64,
        player_state: PlayerState,
        direction: String,
    ) -> Pin<Box<dyn Future<Output=Result<(Location, String), Error>> + Send>> {
        let passage_query_clone = self.passage_query.clone();
        let location_query_clone = self.location_query.clone();

        async move {
            let passage_result = passage_query_clone
                .find_passage_by_location_and_direction(player_state.current_location_id(), &direction)
                .await;

            match passage_result {
                Ok(Some(passage)) => {
                    let location_result = location_query_clone
                        .get_location_by_aggregate_id(
                            game_id,
                            passage.get_to_location()
                        ).await;

                    match location_result {
                        Ok(Some(target_location)) => {
                            let narration = format!("{} and reach {}.", passage.narration(), target_location.title());
                            Ok((target_location, narration))
                        }
                        Ok(None) => Err(NO_ENTRY_FOUND.instantiate(vec![
                            "location".to_string(),
                            "current player state".to_string(),
                        ])),
                        Err(e) => Err(e),
                    }
                }
                Ok(None) => Err(NO_ENTRY_FOUND.instantiate(vec![
                    "passage".to_string(),
                    format!("direction: {}", direction.clone()),
                ])),
                Err(e) => Err(e),
            }
        }
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use std::{fmt, future};
    use futures::future::BoxFuture;
    use mockall::mock;
    use mockall::predicate::eq;

    use domain_pure::model::location::{Location, LocationBuilder};
    use domain_pure::model::passage::PassageBuilder;
    use domain_pure::model::passage::Passage;
    use domain_pure::model::player_state::PlayerStateBuilder;

    use super::*;

    mock! {
        LocationQueries {}
        impl LocationQueries for LocationQueries {
            fn get_location_by_aggregate_id
                (
                    &self,
                    game_id: u64,
                    location_aggregate_id: u64
                ) -> Pin<Box<dyn Future<Output = Result<Option<Location>, Error>> + Send>>;
        }
    }

    impl fmt::Debug for MockLocationQueries {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("MockLocationQueries")
                .finish()  // Adjust according to what you might want to show in debug
        }
    }

    mock! {
        PassageQueries {}
        impl PassageQueries for PassageQueries {
            fn find_passage_between_locations(&self, from_location_id: u64, to_location_id: u64) -> BoxFuture<'static, Result<Option<Passage>, Error>>;
            fn find_passage_by_location_and_direction(&self, location_id: u64, direction: &str) -> BoxFuture<'static, Result<Option<Passage>, Error>>;
        }
    }

    impl fmt::Debug for MockPassageQueries {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("PassageQueries")
                .finish()  // Adjust according to what you might want to show in debug
        }
    }


    #[tokio::test]
    async fn navigate_to_passage_success() {
        let mut mock_passage_query = MockPassageQueries::new();
        let mut mock_location_query = MockLocationQueries::new();

        mock_location_query.expect_get_location_by_aggregate_id()
            .with(eq(2_u64))
            .times(1)
            .returning(|_|
                future::ready(
                    Ok(
                        Some(LocationBuilder::default()
                            .aggregate_id(2)
                            .title("Target Location".into())
                            .description("Description of Target Location".into())
                            .build().unwrap())
                    )
                ).boxed()
            );

        mock_passage_query.expect_find_passage_by_location_and_direction()
            .with(eq(1), eq("north"))
            .times(1)
            .returning(|_, _|
                future::ready(Ok(
                    Some(PassageBuilder::default()
                        .aggregate_id(2)
                        .from_location_id(1)
                        .to_location_id(2)
                        .description("Description of passage".into())
                        .direction("north".into())
                        .narration("You go north".into())
                        .build().unwrap())
                )).boxed()
            );

        let navigation_service = NavigationService::new(Arc::new(mock_location_query), Arc::new(mock_passage_query));
        let player_state_instance = PlayerStateBuilder::default().player_id(1).current_location_id(1).build().unwrap();
        let result = navigation_service.navigate(player_state_instance, "north".into()).await;

        assert!(result.is_ok());
        let (location, narration) = result.unwrap();
        assert_eq!(location.aggregate_id(), 2);
        assert_eq!(narration, "You go north and reach Target Location.");
    }
}
