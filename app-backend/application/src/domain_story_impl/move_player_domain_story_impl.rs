use std::sync::Arc;

use futures::future::BoxFuture;

use domain_contract::services::navigation_services::{NavigationService, NavigationServiceTrait};
use domain_contract::services::player_state_service::PlayerStateService;
use port::context::RequestContext;
use port::port_services::domain_story_move_player::{MovePlayerCommand, MovePlayerDomainStory, MovePlayerResult};
use port::repositories::bout_repository::BoutRepository;
use port::repositories::location_repository::LocationRepository;
use port::repositories::passage_repository::PassageRepository;
use port::repositories::player_state_repository::PlayerStateRepository;

use crate::contract_implementations::bout_query_impl::BoutQueryImpl;
use crate::contract_implementations::location_query_impl::LocationQueryImpl;
use crate::contract_implementations::passage_query_impl::PassageQueryImpl;
use crate::contract_implementations::player_state_query_impl::PlayerStateQueryImpl;
use crate::dto_domain_mapping::player_state_mapper::player_state_map_domain_to_dto;

#[derive(Clone, Debug)]
pub struct MovePlayerDomainStoryImpl {
    location_repository: Arc<dyn LocationRepository>,
    passage_repository: Arc<dyn PassageRepository>,
    bout_repository: Arc<dyn BoutRepository>,
    player_state_repository: Arc<dyn PlayerStateRepository>,
    navigation_service: Arc<dyn NavigationServiceTrait>,
    player_state_service: Arc<PlayerStateService>,  // Added PlayerStateService
}

impl MovePlayerDomainStoryImpl {
    pub fn new(
        location_repository: Arc<dyn LocationRepository>,
        passage_repository: Arc<dyn PassageRepository>,
        bout_repository: Arc<dyn BoutRepository>,
        player_state_repository: Arc<dyn PlayerStateRepository>,
        //  player_state_service: Arc<PlayerStateService>,  // Initialize PlayerStateService
    ) -> Self {
        let navigation_service = Arc::new(
            NavigationService::new(
                Arc::new(LocationQueryImpl::new(location_repository.clone())),
                Arc::new(PassageQueryImpl::new(passage_repository.clone())))
        );


        let player_state_service = Arc::new(
            PlayerStateService::new(
                Arc::new(BoutQueryImpl::new(bout_repository.clone())),
                Arc::new(PlayerStateQueryImpl::new(player_state_repository.clone())))
        );

        Self {
            location_repository,
            passage_repository,
            bout_repository,
            player_state_repository,
            navigation_service,
            player_state_service,
        }
    }
}

impl MovePlayerDomainStory for MovePlayerDomainStoryImpl {
    fn execute(&self, context: RequestContext, input: MovePlayerCommand) -> BoxFuture<'static, Result<MovePlayerResult, String>> {
        let player_state_service_clone = self.player_state_service.clone();
        let navigation_service_clone = self.navigation_service.clone();
        let player_repo_clone = self.player_state_repository.clone();

        Box::pin(async move {
            let player_state_result = player_state_service_clone.get_or_initialize_player_state(context.bout_id, context.player_id).await;

            match player_state_result {
                Ok(mut player_state) => {
                    let navigation_result = navigation_service_clone.navigate(
                        context.bout_id, // Assuming bout_id correlates with game_id or make necessary adjustments
                        player_state.clone(),
                        input.direction,
                    ).await;

                    match navigation_result {
                        Ok((new_location, narration)) => {
                            // Update player state with new location and save
                            player_state.set_current_location_id(new_location.aggregate_id());
                            let _ = player_repo_clone.save(player_state_map_domain_to_dto(&player_state)).await; // Persist the updated state

                            Ok(MovePlayerResult {
                                player_location: new_location.aggregate_id(),
                                narration,
                            })
                        }
                        Err(e) => Err(e.to_string()),
                    }
                }
                Err(e) => Err(e.to_string()),
            }
        })
    }
}
