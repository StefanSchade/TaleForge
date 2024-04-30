use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crosscutting::error_management::error::Error;
use crosscutting::error_management::standard_errors::NO_ENTRY_FOUND;
use domain_pure::model::location::Location;
use domain_pure::model::passage::Passage;
use domain_pure::model::player_state::PlayerState;

use crate::contracts::location_query::navigation::LocationQueries;
use crate::contracts::passage_query::PassageQueries;

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

        Box::pin(async move {
            let passage_result = Self::find_passage(passage_query_clone, game_id, &player_state, &direction).await;
            Self::process_passage_result(passage_result, game_id, location_query_clone, direction).await
        })
    }
}

impl NavigationService {
    async fn find_passage(
        passage_query: Arc<dyn PassageQueries>,
        game_id: u64,
        player_state: &PlayerState,
        direction: &str,
    ) -> Result<Option<Passage>, Error> {
        passage_query.find_passage_by_location_and_direction(game_id, player_state.current_location_id(), direction).await
    }

    async fn process_passage_result(
        passage_result: Result<Option<Passage>, Error>,
        game_id: u64,
        location_query: Arc<dyn LocationQueries>,
        direction: String,
    ) -> Result<(Location, String), Error> {
        match passage_result {
            Ok(Some(passage)) => {
                let location_result = location_query.get_location_by_game_id_and_aggregate_id(game_id, passage.get_to_location()).await;
                NavigationService::handle_location_result(location_result, passage)
            }
            Ok(None) => Err(NO_ENTRY_FOUND.instantiate(vec![
                "passage".to_string(),
                format!("direction: {}", direction),
            ])),
            Err(e) => Err(e),
        }
    }

    fn handle_location_result(
        location_result: Result<Option<Location>, Error>,
        passage: Passage,
    ) -> Result<(Location, String), Error> {
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
}

#[cfg(test)]
mod tests {
    use std::{fmt, future};

    use futures::future::BoxFuture;
    use futures::FutureExt;
    use mockall::mock;
    use mockall::predicate::eq;

    use domain_pure::model::location::{Location, LocationBuilder};
    use domain_pure::model::passage::Passage;
    use domain_pure::model::passage::PassageBuilder;
    use domain_pure::model::player_state::PlayerStateBuilder;

    use super::*;

    mock! {
        LocationQueries {}
        impl LocationQueries for LocationQueries {
            fn get_location_by_game_id_and_aggregate_id
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
            fn find_passage_between_locations(&self, game_id: u64, from_location_id: u64, to_location_id: u64) -> BoxFuture<'static, Result<Option<Passage>, Error>>;
            fn find_passage_by_location_and_direction(&self, game_id: u64, location_id: u64, direction: &str) -> BoxFuture<'static, Result<Option<Passage>, Error>>;
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
        let expected_origination_location = 1;
        let expected_destination_location = 2;
        let expected_game_id = 10;
        let expected_bout_id: u64 = 15;
        let expected_passage_id = 20;

        let mut mock_passage_query = MockPassageQueries::new();
        let mut mock_location_query = MockLocationQueries::new();

        // Clone all necessary data before defining the returning closure
        let origination_location_clone = expected_origination_location;
        let destination_location_clone = expected_destination_location;
        let game_id_clone = expected_game_id;
        let passage_id_clone = expected_passage_id;

        // Setup mock for location query
        mock_location_query.expect_get_location_by_game_id_and_aggregate_id()
            .with(eq(game_id_clone), eq(destination_location_clone))
            .times(1)
            .returning(move |_, _| {
                future::ready(Ok(
                    Some(LocationBuilder::default()
                        .aggregate_id(destination_location_clone)
                        .game_id(game_id_clone)
                        .title("Target Location".to_string())
                        .description("Description of Target Location".to_string())
                        .build().unwrap())
                )).boxed()
            });

        // Setup mock for passage query
        mock_passage_query.expect_find_passage_by_location_and_direction()
            .with(eq(expected_game_id), eq(expected_origination_location), eq("north"))
            .times(1)
            .returning(move |_, _, _| {
                future::ready(Ok(
                    Some(PassageBuilder::default()
                        .aggregate_id(passage_id_clone)
                        .game_id(game_id_clone)
                        .from_location_id(origination_location_clone)
                        .to_location_id(destination_location_clone)
                        .description("Description of passage".to_string())
                        .direction("north".to_string())
                        .narration("You go north".to_string())
                        .build().unwrap())
                )).boxed()
            });

        let navigation_service = NavigationService::new(
            Arc::new(mock_location_query),
            Arc::new(mock_passage_query));

        let player_state_instance = PlayerStateBuilder::default()
            .player_id(1)
            .bout_id(expected_bout_id)
            .current_location_id(expected_origination_location)
            .build().unwrap();

        let result = navigation_service.navigate(expected_game_id, player_state_instance, "north".to_string()).await;

        assert!(result.is_ok());
        let (location, narration) = result.unwrap();
        assert_eq!(location.aggregate_id(), expected_destination_location);
        assert_eq!(narration, "You go north and reach Target Location.");
    }
}