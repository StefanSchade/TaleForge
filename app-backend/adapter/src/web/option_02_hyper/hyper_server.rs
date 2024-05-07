use std::future::Future;
use std::io::Error;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

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
use crate::web::option_01_actixweb::app_state::AppState;

#[derive(Clone, Debug)]
pub struct HyperServer {
    service_container: ServiceContainer,
    config: Arc<ServerConfig>,
}

impl HyperServer {
    pub fn new(container: ServiceContainer, config: ServerConfig) -> Self {
        HyperServer {
            service_container: container,
            config: Arc::new(config)
        }
    }
}
impl WebServer for HyperServer {
    fn start_server(&self) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
        let app_state = AppState {
            move_player_domain_story: self.service_container.move_player(),
        };

        let config_clone = self.config.clone();

        Box::pin(async move {
            // Directly parse the address without taking a reference
            let addr = match config_clone.address.parse::<SocketAddr>() {
                Ok(addr) => addr,
                Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidInput, format!("Failed to parse bind address: {}", e)))
            };

            if config_clone.use_https {
                // HTTPS setup
                #[cfg(feature = "ssl")]
                {
                    let ssl_key = config_clone.ssl_key.expect("SSL key file is required for HTTPS");
                    let ssl_cert = config_clone.ssl_cert.expect("SSL cert file is required for HTTPS");

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

pub trait DummyContextTrait: Send + Sync {}

#[derive(Debug, Clone, Copy)]
pub struct DummyContext {}

impl DummyContextTrait for DummyContext {}
