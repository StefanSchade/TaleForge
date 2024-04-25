use std::sync::Arc;

use futures::future::BoxFuture;

use domain_contract::services::navigation_services::{NavigationService, NavigationServiceTrait};
use port::context::RequestContext;
use port::port_services::domain_story_move_player::{MovePlayerCommand, MovePlayerDomainStory, MovePlayerResult};
use port::port_services::domain_story_move_player2::MovePlayerDomainStory2;
use port::repositories::location_repository::LocationRepository;
use port::repositories::passage_repository::PassageRepository;
use port::repositories::player_state_repository::PlayerStateRepository;

use crate::contract_implementations::location_query_impl::LocationQueryImpl;
use crate::contract_implementations::passage_query_impl::PassageQueryImpl;
use crate::dto_domain_mapping::player_state_mapper::player_state_map_dto_to_domain;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MovePlayerDomainStoryImpl2 {
    location_repository: Arc<dyn LocationRepository>,
    passage_repository: Arc<dyn PassageRepository>,
    player_state_repository: Arc<dyn PlayerStateRepository>,
    navigation_service: Arc<dyn NavigationServiceTrait>,
}

impl MovePlayerDomainStoryImpl2 {
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
            passage_repository,
            player_state_repository,
            navigation_service,
        }
    }
}

impl MovePlayerDomainStory2 for MovePlayerDomainStoryImpl2 {
    fn execute(&self, context: RequestContext, input: MovePlayerCommand) -> BoxFuture<'static, Result<MovePlayerResult, String>> {
        let player_repo_clone = self.player_state_repository.clone();
        let navigation_service_clone = self.navigation_service.clone();
        Box::pin(async move {
            if let Some(player_id) = context.player_id {
                let player_state_dto = match player_repo_clone.find_by_player_id(player_id).await {
                    Ok(Some(state)) => state,
                    Ok(None) => return Err("Player state not found".to_string()),
                    Err(e) => return Err(e.to_string()),
                };

                let player_state = player_state_map_dto_to_domain(&player_state_dto);
                let (new_location, narration) = match navigation_service_clone.navigate(player_state, input.direction).await {
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
