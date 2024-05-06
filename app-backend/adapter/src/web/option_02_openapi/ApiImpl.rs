use std::sync::Arc;

use swagger::ApiError;

use openapi_client::Api;
use openapi_client::models::{MovePlayerRequest, MovePlayerResponse};
use port::adapters_outbound::service_container::ServiceContainer;
use port::port_services::domain_story_move_player::MovePlayerDomainStory;

use crate::web::shared::domain_story_mappers::PlayerMoveRequestMapper::PlayerMoveRequestMapper;
use crate::web::shared::domain_story_mappers::PlayerMoveResonseMapper::PlayerMoveResponseMapper;
use crate::web::shared::MapperTrait::MapperTrait;

//impl<B: BusinessAdapter + Send + Sync> Api<RequestContext> for ApiAdapter<B>

#[derive(Debug, Clone)]
pub struct Adapter {
    move_player_domain_story: Arc<dyn MovePlayerDomainStory + Send + Sync>,
}

impl Adapter {
    pub fn new(move_player_domain_story: Arc<dyn MovePlayerDomainStory + Send + Sync>) -> Self {
        Self {
            move_player_domain_story,
        }
    }
}

trait DummyContextTrait: Send + Sync {}

#[derive(Debug, Clone, Copy)]
struct DummyContext {}

impl DummyContextTrait for DummyContext {}

impl Api<DummyContext> for Adapter {
    async fn move_player(&self, move_player_request: MovePlayerRequest, context: &DummyContext) -> Result<MovePlayerResponse, ApiError> {
        let domain_story = self.move_player_domain_story.clone();
        let _context_clone = context.clone();
        match PlayerMoveRequestMapper::from_api(move_player_request) {
            Ok(request) => {
                match domain_story.execute(request).await {
                    Ok(response) => {
                        match PlayerMoveResponseMapper::to_api(response) {
                            Ok(api_response) => Ok(api_response),
                            Err(mapping_error) =>
                                Err(ApiError(format!("Failed to map domain response to API response: {}", mapping_error)))
                        }
                    }
                    Err(e) => {
                        Err(ApiError(format!("Error processing move player request: {}", e)))
                    }
                }
            }
            Err(e) => {
                Err(ApiError(format!("Error processing move player request: {}", e)))
            }
        }
    }
}