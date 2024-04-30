use std::sync::Arc;

use futures::future::BoxFuture;

use domain_contract::services::navigation_services::{NavigationService, NavigationServiceTrait};
use port::context::RequestContext;
use port::dto::bout_dto::BoutStatusDTO;
use port::port_services::domain_story_move_player::{MovePlayerCommand, MovePlayerDomainStory, MovePlayerResult};
use port::repositories::bout_repository::BoutRepository;
use port::repositories::location_repository::LocationRepository;
use port::repositories::passage_repository::PassageRepository;
use port::repositories::player_state_repository::PlayerStateRepository;

use crate::contract_implementations::location_query_impl::LocationQueryImpl;
use crate::contract_implementations::passage_query_impl::PassageQueryImpl;
use crate::dto_domain_mapping::player_state_mapper::player_state_map_dto_to_domain;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MovePlayerDomainStoryImpl {
    pub(crate) location_repository: Arc<dyn LocationRepository>,
    pub(crate) passage_repository: Arc<dyn PassageRepository>,
    pub(crate) player_state_repository: Arc<dyn PlayerStateRepository>,
    pub(crate) navigation_service: Arc<dyn NavigationServiceTrait>,
    pub(crate) bout_repository: Arc<dyn BoutRepository>,
}

impl MovePlayerDomainStoryImpl {
    pub fn new(
        location_repository: Arc<dyn LocationRepository>,
        passage_repository: Arc<dyn PassageRepository>,
        bout_repository: Arc<dyn BoutRepository>,
        player_state_repository: Arc<dyn PlayerStateRepository>,
    ) -> Self {
        let navigation_service = Arc::new(
            NavigationService::new(
                Arc::new(LocationQueryImpl::new(location_repository.clone())),
                Arc::new(PassageQueryImpl::new(passage_repository.clone())))
        );

        Self {
            location_repository,
            player_state_repository,
            passage_repository,
            bout_repository,
            navigation_service,
        }
    }
}

impl MovePlayerDomainStory for MovePlayerDomainStoryImpl {
    fn execute(&self, context: RequestContext, input: MovePlayerCommand) -> BoxFuture<'static, Result<MovePlayerResult, String>> {
        let player_repo_clone = self.player_state_repository.clone();
        let navigation_service_clone = self.navigation_service.clone();
        let bout_repo_clone = self.bout_repository.clone();

        Box::pin(async move {
            let current_bout = bout_repo_clone.get_bout_by_id(context.bout_id).await;
            match current_bout {
                Ok(Some(bout)) if bout.status == BoutStatusDTO::Running => {
                    let game_id = bout.game_id;
                    let player_id = context.player_id;

                    let player_state_dto = player_repo_clone.find_by_player_id(player_id).await;
                    match player_state_dto {
                        Ok(Some(state)) => {
                            let player_state = player_state_map_dto_to_domain(&state);

                            let navigation_result = navigation_service_clone.navigate(
                                game_id,
                                player_state,
                                input.direction,
                            ).await;

                            match navigation_result {
                                Ok((new_location, narration)) => {
                                    let mut updated_player_state = state;
                                    updated_player_state.current_location_id = new_location.aggregate_id();
                                    player_repo_clone.save(updated_player_state).await
                                        .map(|_| MovePlayerResult {
                                            player_location: new_location.aggregate_id(),
                                            narration,
                                        })
                                        .map_err(|e| e.to_string())
                                }
                                Err(e) => Err(e.to_string()),
                            }
                        }
                        Ok(None) => Err(format!("Player state not found for player {:?}", player_id)),
                        Err(e) => Err(e.to_string()),
                    }
                }
                Ok(Some(bout)) => {
                    Err(format!("Bout with ID {} is not running, status is {:?}", bout.id, bout.status))
                }
                Ok(None) => Err("Bout not found".to_string()),
                Err(e) => Err(e.to_string()),
            }
        })
    }
}


#[cfg(test)]
mod tests {
    use fmt::Debug;
    use std::fmt;

    use futures::FutureExt;
    use mockall::{mock, predicate::*};
    use mockall::predicate::eq;

    use crosscutting::error_management::error::Error;
    use port::dto::bout_dto::{BoutDTO, BoutStatusDTO};
    use port::dto::location_dto::LocationDTO;
    use port::dto::passage_dto::PassageDTO;
    use port::dto::player_state_dto::PlayerStateDTO;
    use port::repositories::location_repository::LocationRepository;

    use super::*;

    mock! {
        LocationRepository {}

        impl LocationRepository for LocationRepository  {
            fn get_location_by_id(&self, game: u64,  id: u64) -> BoxFuture<'static, Result<Option<LocationDTO>, Error>>;
            fn get_all_locations(&self, game: u64) -> BoxFuture<'static, Result<Vec<LocationDTO>, Error>>;
            fn add_location(&self, game: u64, location: LocationDTO) -> BoxFuture<'static, Result<(), Error>>;
        }
    }

    impl Debug for MockLocationRepository {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("MockLocationRepository")
                .finish()  // Adjust according to what you might want to show in debug
        }
    }

    mock! {
        PassageRepository {}

         impl  PassageRepository for PassageRepository {
            fn find_passage_by_location_and_direction(&self,game_id: u64,  location_id: u64, direction: &str) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>>;
            fn get_passage_by_id(&self,game_id: u64,  id: u64) ->  BoxFuture<'static, Result<Option<PassageDTO>, Error>>;
            fn get_passages_for_location(&self,game_id: u64,  location_id: u64) ->  BoxFuture<'static, Result<Vec<PassageDTO>, Error>>;
            fn add_passage(&self,game_id: u64,  passage: PassageDTO) ->  BoxFuture<'static, Result<(), Error>>;
            fn find_by_start_and_end_id(&self,game_id: u64,  from_location_id: u64, to_location_id:u64) ->  BoxFuture<'static, Result<Option<PassageDTO>, Error>>;
        }
    }

    impl Debug for MockPassageRepository {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("MockPassageRepository")
                .finish()  // Adjust according to what you might want to show in debug
        }
    }

    mock! {
        PlayerStateRepository {}

        impl PlayerStateRepository for PlayerStateRepository  {
             fn find_by_player_id(&self, id: u64) -> BoxFuture<'static, Result<Option<PlayerStateDTO>, Error>>;
             fn save(&self, player_state: PlayerStateDTO) -> BoxFuture<'static, Result<Option<PlayerStateDTO>, Error>>;
        }
    }

    impl fmt::Debug for MockPlayerStateRepository {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("MockPlayerStateRepository")
                .finish()  // Adjust according to what you might want to show in debug
        }
    }

    mock! {
        BoutRepository {}

        impl BoutRepository for BoutRepository  {
            fn get_bout_by_id(&self, match_id: u64) -> BoxFuture<'static, Result<Option<BoutDTO>, Error>>;
            fn add_bout(&self, bout: BoutDTO) -> BoxFuture<'static, Result<(), Error>>;
        }
    }

    impl Debug for MockBoutRepository {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("BoutRepository")
                .finish()
        }
    }

    #[tokio::test]
    async fn test_move_player_success() {
        let expected_destination_location_id: u64 = 99;
        let expected_passage_narration_text: &str = "You take the passage and reach the new location.";
        let expected_direction_instruction: &str = "north";
        let expected_player_id: u64 = 42;

        let mut mock_location_repo = MockLocationRepository::new();
        let mut mock_passage_repo = MockPassageRepository::new();
        let mut mock_player_state_repo = MockPlayerStateRepository::new();
        let mut mock_bout_repo = MockBoutRepository::new();

        let expected_current_location_id: u64 = 74;
        let expected_destination_location_id: u64 = 75;
        let expected_game_id: u64 = 45;
        let expected_bout_id: u64 = 43;
        let expected_player_id: u64 = 66;

        mock_passage_repo.expect_find_passage_by_location_and_direction()
            .with(eq(expected_game_id),eq(expected_current_location_id), eq("north"))
            .times(1)
            .returning(move |_, _, _|
                async move {
                    Ok(
                        Some(PassageDTO {
                            id: 1,
                            game_id: 2,
                            description: "north".to_string(),
                            direction: "north".to_string(),
                            narration: "You take the passage".to_string(),
                            from_location_id: expected_current_location_id,
                            to_location_id: expected_destination_location_id,
                        })
                    )
                }.boxed()
            );

        mock_location_repo.expect_get_location_by_id()
            .with(eq(expected_game_id), eq(expected_destination_location_id))
            .times(1)
            .returning(move |_, _|
                async move {
                    Ok(
                        Some(LocationDTO {
                            id: expected_destination_location_id,
                            game_id: expected_game_id,
                            title: "the new location".to_string(),
                            description: "destination".to_string(),
                            image_url: None,
                        })
                    )
                }.boxed()
            );

        mock_bout_repo.expect_get_bout_by_id()
            .with(eq(expected_bout_id))
            .times(1)
            .returning(move |_|
                async move {
                    Ok(
                        Some(
                            BoutDTO {
                                id: expected_bout_id,
                                game_id: expected_game_id,
                                registered_participants: vec![1001_u64, expected_player_id, 1003_u64],
                                status: BoutStatusDTO::Running,
                            }
                        )
                    )
                }.boxed()
            );

        mock_player_state_repo.expect_find_by_player_id()
            .with(eq(expected_player_id))
            .times(1)
            .returning(move |_|
                async move {
                    Ok(
                        Some(
                            PlayerStateDTO {
                                player_id: expected_player_id,
                                bout_id: expected_bout_id,
                                current_location_id: expected_current_location_id,
                            }
                        )
                    )
                }.boxed()
            );

        // `expected_player_id` is of type u64 and thus implements the `Copy` trait, implying that instead of
        // borrowing it, it will be copied by simply passing it without explicitly calling `.clone()`
        // However doing so results in an [E0373], so we apply what we would do with non-Copy data and move
        // ownership explicitly to the closure.
        mock_player_state_repo.expect_save()
            .withf(move |ps| ps.player_id == expected_player_id)
            .times(1)
            .returning(|_| async { Ok(None) }.boxed());


        let use_case =
            MovePlayerDomainStoryImpl::new(
                Arc::new(mock_location_repo),
                Arc::new(mock_passage_repo),
                Arc::new(mock_bout_repo),
                Arc::new(mock_player_state_repo),
            );

        let command = MovePlayerCommand { direction: expected_direction_instruction.into() };
        let context = RequestContext { bout_id: 43, player_id: expected_player_id };

        let result = use_case.execute(context, command).await;

        assert!(result.is_ok());

        let move_player_result = result.unwrap();

        assert_eq!(move_player_result.player_location, expected_destination_location_id);
        assert_eq!(move_player_result.narration, expected_passage_narration_text.to_string());
    }
}