use std::future::Future;
use std::io::Error;
use std::net::SocketAddr;
use std::pin::Pin;
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
use port::adapters_inbound::web_server::{ServerConfig, WebServer};
use port::adapters_outbound::service_container::ServiceContainer;
use port::port_services::domain_story_move_player::MovePlayerDomainStory;

use crate::web::shared::domain_story_mappers::player_move_request_mapper::PlayerMoveRequestMapper;
use crate::web::shared::domain_story_mappers::player_move_resonse_mapper::PlayerMoveResponseMapper;
use crate::web::shared::request_mapper_trait::RequestMapperTrait;
use crate::web::shared::response_mapper_trait::ResponseMapperTrait;

#[derive(Clone, Debug)]
pub struct HyperServer {
    move_player_domain_story: Arc<dyn MovePlayerDomainStory>,
}

impl HyperServer {
    pub fn new(service_container: ServiceContainer) -> Self {
        let move_player_domain_story = service_container.move_player();
        HyperServer {
            move_player_domain_story,
        }
    }
}

impl WebServer for HyperServer {
    fn start_server(&self, config: ServerConfig) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
        Box::pin(async move {
            let addr_result = config.address.parse::<SocketAddr>();
            let addr = match addr_result {
                Ok(addr) => addr,
                Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidInput, format!("Failed to parse bind address: {}", e)))
            };
            if config.use_https {
                // HTTPS setup
                #[cfg(feature = "ssl")]
                {
                    let ssl_key = config.ssl_key.expect("SSL key file is required for HTTPS");
                    let ssl_cert = config.ssl_cert.expect("SSL cert file is required for HTTPS");

                    // Here would be your SSL setup code
                    // Placeholder for SSL code
                }

                // Placeholder for the actual HTTPS server setup
                println!("HTTPS server started on {}", addr);
                Ok(())
            } else {
                // HTTP setup
                let make_svc = hyper::service::make_service_fn(|_conn| async {
                    Ok::<_, hyper::Error>(hyper::service::service_fn(|_req| async {
                        Ok::<_, hyper::Error>(hyper::Response::new(hyper::Body::from("Hello World!")))
                    }))
                });

                let server = hyper::server::Server::bind(&addr).serve(make_svc);
                println!("HTTP server started on {}", addr);
                server.await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Hyper error: {}", e)))
            }
        })
    }
}

trait DummyContextTrait: Send + Sync {}

#[derive(Debug, Clone, Copy)]
struct DummyContext {}

impl DummyContextTrait for DummyContext {}

#[async_trait]
impl Api<DummyContext> for HyperServer {
    async fn move_player(&self, move_player_request: ModelMovePlayerRequest, context: &DummyContext) -> Result<MovePlayerResponseCodesAndBody, ApiError> {
        let domain_story = self.move_player_domain_story.clone();
        let _context_clone = context.clone();

        match PlayerMoveRequestMapper::from_api(move_player_request) {
            Ok(request) => {
                match domain_story.execute(request).await {
                    Ok(response) => {
                        // Since the line was simple, no need for additional braces
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
