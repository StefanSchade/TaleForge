use std::sync::Arc;

use swagger::ApiError;

use openapi_client::{Api, models};
use openapi_client::models::{MovePlayerResponse, MovePlayerRequest};
use port::adapters_outbound::service_container::ServiceContainer;
use port::port_services::domain_story_move_player::{MovePlayerDomainStory, MovePlayerDomainStoryRequest, MovePlayerDomainStoryResponse};

//impl<B: BusinessAdapter + Send + Sync> Api<RequestContext> for ApiAdapter<B>

#[derive(Debug, Clone)]
struct Adapter {
    move_player_domain_story: Arc<dyn MovePlayerDomainStory>,
}

impl Adapter {
    pub fn new(service_container: ServiceContainer) -> Self {
        let move_player_domain_story = service_container.move_player().clone();

        Adapter {
            move_player_domain_story
        }
    }
}

trait DummyContextTrait: Send + Sync {}

#[derive(Debug, Clone, Copy)]
struct DummyContext {}

impl DummyContextTrait for DummyContext {}

impl Api<DummyContext> for Adapter {
    async fn move_player(&self, move_player_request: MovePlayerRequest, context: &DummyContext) -> Result<MovePlayerResponse, ApiError> {

        let request_port = Self::move_player_ds_request_to_port(move_player_request);
        let domain_story = self.move_player_domain_story.clone();
        let _context_clone = context.clone();

        // Execute the domain story
        match domain_story.execute(request_port).await {
            Ok(domain_response) => {
                let api_response = Self::move_player_ds_response_to_openApi(domain_response);
                Ok(MovePlayerResponse::PlayerMovedSuccessfully(api_response))
            }
            Err(e) => {
                Err(ApiError(format!("Error processing move player request: {}", e)))
            }
        }
    }
}

impl Adapter {
    pub fn move_player_ds_response_to_openApi(port_definition: MovePlayerDomainStoryResponse) -> MovePlayerResponse {
        MovePlayerResponse {
            player_location: Option::from(port_definition.player_location),
            narration: Option::from(port_definition.narration),
        }
    }

    pub fn move_player_ds_response_to_port(openapi: MovePlayerResponse) -> MovePlayerDomainStoryResponse {
        MovePlayerDomainStoryResponse {
            player_location: openapi.player_location.unwrap_or_default(),
            narration: openapi.narration.unwrap_or_default(),
        }
    }

    pub fn move_player_ds_request_to_openapi(port_definition: MovePlayerDomainStoryRequest) -> MovePlayerRequest {
        MovePlayerRequest {
            bout_id: port_definition.bout_id,
            player_id: port_definition.player_id,
            direction: port_definition.direction,
        }
    }

    pub fn move_player_ds_request_to_port(openapi: MovePlayerRequest) -> MovePlayerDomainStoryRequest {
        MovePlayerDomainStoryRequest {
            direction: openapi.direction,
            bout_id: openapi.bout_id,
            player_id: openapi.player_id,
        }
    }
}