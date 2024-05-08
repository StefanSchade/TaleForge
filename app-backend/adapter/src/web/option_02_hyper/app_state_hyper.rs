use std::sync::Arc;

use async_trait::async_trait;
use swagger::ApiError;

use openapi_client::Api;
#[allow(unused_imports)]
use openapi_client::models::Error as ErrorBody;
#[allow(unused_imports)]
use openapi_client::models::MovePlayerRequest as ModelMovePlayerRequest;
#[allow(unused_imports)]
use openapi_client::models::MovePlayerResponse as MovePlayerResponseBody;
#[allow(unused_imports)]
use openapi_client::MovePlayerResponse as MovePlayerResponseCodesAndBody;
use port::adapters_outbound::service_container::ServiceContainer;
use port::port_services::domain_story_move_player::MovePlayerDomainStory;

use crate::web::shared::domain_story_mappers::player_move_request_mapper::PlayerMoveRequestMapper;
use crate::web::shared::domain_story_mappers::player_move_resonse_mapper::PlayerMoveResponseMapper;
use crate::web::shared::request_mapper_trait::RequestMapperTrait;
use crate::web::shared::response_mapper_trait::ResponseMapperTrait;

#[derive(Clone, Debug)]
pub struct AppStateHyper {
    pub move_player_domain_story: Arc<dyn MovePlayerDomainStory>,
}

impl AppStateHyper {
    pub fn new(move_player_domain_story: Arc<dyn MovePlayerDomainStory>) -> Self {
        AppStateHyper {
            move_player_domain_story,
        }
    }
}

#[async_trait]
impl Api<crate::web::option_02_hyper::hyper_server::DummyContext> for AppStateHyper {
    async fn move_player
    (
        &self,
        move_player_request: ModelMovePlayerRequest,
        context: &crate::web::option_02_hyper::hyper_server::DummyContext,
    ) -> Result<MovePlayerResponseCodesAndBody, ApiError>
    {
        let domain_story = self.move_player_domain_story.clone();
        let _context_clone = context.clone();

        match PlayerMoveRequestMapper::from_api(move_player_request) {
            Ok(request) => {
                match domain_story.execute(request).await {
                    Ok(response) => {
                        Ok(PlayerMoveResponseMapper::to_api_response_codes(response))
                    }
                    Err(e) => {
                        Err(ApiError(format!("Error processing domain story: {}", e)))
                    }
                }
            }
            Err(e) => {
                Err(ApiError(format!("Error processing move player request: {}", e)))
            }
        }
    }
}