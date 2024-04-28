use std::sync::Arc;

use futures::future::BoxFuture;

use domain_contract::services::navigation_services::{NavigationService, NavigationServiceTrait};
use port::context::RequestContext;
use port::port_services::domain_story_move_player::{MovePlayerCommand, MovePlayerDomainStory, MovePlayerResult};
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
    // unused currently
    pub(crate) passage_repository: Arc<dyn PassageRepository>,
    pub(crate) player_state_repository: Arc<dyn PlayerStateRepository>,
    pub(crate) navigation_service: Arc<dyn NavigationServiceTrait>,
}

impl MovePlayerDomainStoryImpl {
    pub fn new(
        location_repository: Arc<dyn LocationRepository>,
        passage_repository: Arc<dyn PassageRepository>,
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
            navigation_service,
        }
    }
}

impl MovePlayerDomainStory for MovePlayerDomainStoryImpl {
    fn execute(&self, context: RequestContext, input: MovePlayerCommand) -> BoxFuture<'static, Result<MovePlayerResult, String>> {
        let player_repo_clone = self.player_state_repository.clone();
        let navigation_service_clone = self.navigation_service.clone();
        let game_id = context.game_id;
        let _player_id = context.player_id;

        Box::pin(async move {
            if let player_id = context.player_id {
                let player_state_dto = match player_repo_clone.find_by_player_id(player_id).await {
                    Ok(Some(state)) => state,
                    Ok(None) => return Err(format!("Player state not found for player {:?}", player_id)),
                    Err(e) => return Err(e.to_string()),
                };

                let player_state = player_state_map_dto_to_domain(&player_state_dto);
                let (new_location, narration) = match navigation_service_clone.navigate(game_id, player_state, input.direction).await {
                    Ok(result) => result,
                    Err(e) => return Err(e.to_string()),
                };

                let mut updated_player_state = player_state_dto;
                updated_player_state.current_location_id = new_location.aggregate_id();
                player_repo_clone.save(updated_player_state).await.map_err(|e| e.to_string())?;

                Ok(MovePlayerResult {
                    player_location: new_location.aggregate_id(),
                    narration,
                })
            } else {
                Err("Player ID not found in context".to_string())
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
    use domain_pure::model::location::LocationBuilder;
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
            fn find_passage_by_location_and_direction(&self, location_id: u64, direction: &str) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>>;
            fn get_passage_by_id(&self, id: u64) ->  BoxFuture<'static, Result<Option<PassageDTO>, Error>>;
            fn get_passages_for_location(&self, location_id: u64) ->  BoxFuture<'static, Result<Vec<PassageDTO>, Error>>;
            fn add_passage(&self, passage: PassageDTO) ->  BoxFuture<'static, Result<(), Error>>;
            fn find_by_start_and_end_id(&self, from_location_id: u64, to_location_id:u64) ->  BoxFuture<'static, Result<Option<PassageDTO>, Error>>;
        }
    }

    impl Debug for MockPassageRepository {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("MockPassageRepository")
                .finish()  // Adjust according to what you might want to show in debug
        }
    }

    impl fmt::Debug for MockPlayerStateRepository {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("MockPlayerStateRepository")
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

    #[tokio::test]
    async fn test_move_player_success() {
        let expected_destination_location_id: u64 = 99;
        let expected_passage_narration_text: &str = "You take the passage and reach the new location.";
        let expected_direction_instruction: &str = "north";
        let expected_player_id: u64 = 42;

        let mut mock_location_repo = MockLocationRepository::new();
        let mut mock_passage_repo = MockPassageRepository::new();
        let mut mock_player_state_repo = MockPlayerStateRepository::new();

        // We need to pass `expected_location` to the closure `.returning()`, but since this closure might
        // be called multiple times we have to clone it first. Wrapping it into Arc reduces the overhead.
        // We thus guarantee immutability and just have to clone the reference, but not the struct itself.
        let expected_destination_location = Arc::new(LocationBuilder::default()
            .aggregate_id(expected_destination_location_id)
            .game_id(1_u64)
            .title("Destination - This could be anything".to_string())
            .description("Destinatinon Decription - This could be anything".to_string())
            .build()
            .unwrap());

        mock_passage_repo.expect_find_passage_by_location_and_direction()
            .with(mockall::predicate::eq(1), mockall::predicate::eq("north"))
            .times(1)  // You can specify how many times you expect this call
            .returning(|_, _|
                async move {
                    Ok(
                        Some(PassageDTO {
                            id: 1,
                            description: "north".to_string(),
                            direction: "north".to_string(),
                            narration: "You take the passage".to_string(),
                            from_location_id: 1,
                            to_location_id: 1,
                        })
                    )
                }.boxed()
            );

        mock_location_repo.expect_get_location_by_id()
            .with(eq(43), eq(1)) // this is an error 42 should be the game id not the player id
            .times(1)
            .returning(|_, _|
                async move {
                    Ok(
                        Some(LocationDTO {
                            id: 99,
                            game_id: 1,
                            title: "the new location".to_string(),
                            description: "destination".to_string(),
                            image_url: None,
                        })
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
                                current_location_id: 1,
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
                Arc::new(mock_player_state_repo));

        let command = MovePlayerCommand { direction: expected_direction_instruction.into() };
        let context = RequestContext {game_id: 43,  player_id: expected_player_id};

        let result = use_case.execute(context, command).await;

        assert!(result.is_ok());

        let move_player_result = result.unwrap();

        assert_eq!(move_player_result.player_location, expected_destination_location_id);
        assert_eq!(move_player_result.narration, expected_passage_narration_text.to_string());
    }
}
