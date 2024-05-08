use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::io::Error;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use log::info;
use swagger::{ApiError, EmptyContext, Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;

use openapi_client::Api;
#[allow(unused_imports)]
use openapi_client::models::Error as ErrorBody;
#[allow(unused_imports)]
use openapi_client::models::MovePlayerRequest as ModelMovePlayerRequest;
#[allow(unused_imports)]
use openapi_client::models::MovePlayerResponse as MovePlayerResponseBody;
#[allow(unused_imports)]
use openapi_client::MovePlayerResponse as MovePlayerResponseCodesAndBody;
use openapi_client::server::MakeService;
use port::adapters_inbound::web_server::{ServerConfig, WebServer};
use port::adapters_outbound::service_container::ServiceContainer;

use crate::web::shared::domain_story_mappers::player_move_request_mapper::PlayerMoveRequestMapper;
use crate::web::shared::domain_story_mappers::player_move_resonse_mapper::PlayerMoveResponseMapper;
use crate::web::shared::request_mapper_trait::RequestMapperTrait;
use crate::web::shared::response_mapper_trait::ResponseMapperTrait;

#[derive(Clone)]
pub struct HyperServer<C> {
    service_container: ServiceContainer,
    config: Arc<ServerConfig>,
    marker: PhantomData<C>,
}

impl<C> HyperServer<C> {
    pub fn new(container: ServiceContainer, config: ServerConfig) -> Self {
        HyperServer {
            service_container: container,
            config: Arc::new(config),
            marker: PhantomData,
        }
    }
}

#[async_trait]
impl<C> Api<C> for HyperServer<C> where C: Has<XSpanIdString> + Send + Sync {
    async fn move_player
    (
        &self,
        move_player_request: ModelMovePlayerRequest,
        context: &C,
    ) -> Result<MovePlayerResponseCodesAndBody, ApiError>
    {
        let domain_story = self.service_container.move_player().clone();
        let _context_clone = context.clone();
        info!("move_player({:?}) - X-Span-ID: {:?}", move_player_request, context.get().0.clone());

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

impl<C> Debug for HyperServer<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HyperServer").finish()
    }
}

impl<C> WebServer for HyperServer<C> {
    fn start_server(&self) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
        let config_clone = self.config.clone();
        let service = MakeService::new(self);
        let service = MakeAllowAllAuthenticator::new(service, "cosmo");

        #[allow(unused_mut)]
            let mut service =
            openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(
                service
            );

        Box::pin(async move {
            let addr = match config_clone.address.parse::<SocketAddr>() {
                Ok(addr) => addr,
                Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidInput, format!("Failed to parse bind address: {}", e))),
            };

            // Using HTTP
            hyper::server::Server::bind(&addr).serve(service).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        })
    }
}
