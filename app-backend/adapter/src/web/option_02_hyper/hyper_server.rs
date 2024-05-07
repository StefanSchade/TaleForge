use std::convert::Infallible;
use std::future::Future;
use std::io::Error;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use hyper::{Request, Server};

use hyper::service::{make_service_fn, service_fn};

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
            config: Arc::new(config),
        }
    }
}



impl WebServer for HyperServer {
    fn start_server(&self) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {

        // Clone or use Arc to capture data for the async block
        let config_clone = Arc::new(self.config.clone());

        // Ensure app_state or any other referenced state is cloned or wrapped in Arc
        let app_state = AppState {
            move_player_domain_story: self.service_container.move_player(),
        };

        Box::pin(async move {
            let addr = match config_clone.address.parse::<SocketAddr>() {
                Ok(addr) => addr,
                Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidInput, format!("Failed to parse bind address: {}", e))),
            };

            let make_svc = hyper::service::make_service_fn(move |_conn| {
                let app_state_clone = app_state.clone();
                async move {
                    Ok::<_, hyper::Error>(hyper::service::service_fn(move |req| {
                        handle_request(req, app_state_clone.clone())
                    }))
                }
            });

            let server = hyper::server::Server::bind(&addr).serve(make_svc);
            println!("HTTP server started on {}", addr);
            server.await.map_err(|e| Error::new(std::io::ErrorKind::Other, format!("Hyper server error: {}", e)))
        })
    }
}

// Dummy implementation of the handle_request function
async fn handle_request(req: hyper::Request<hyper::Body>, app_state: AppState) -> Result<hyper::Response<hyper::Body>, hyper::Error> {
    // Example handler that just echoes "Hello World!"
    Ok(hyper::Response::new(hyper::Body::from("Hello World!")))
}
        // let config_clone = self.config.clone();
        //
        // Box::pin(async move {
        //     // Directly parse the address without taking a reference
        //     let addr = match config_clone.address.parse::<SocketAddr>() {
        //         Ok(addr) => addr,
        //         Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidInput, format!("Failed to parse bind address: {}", e)))
        //     };
        //
        //     if config_clone.use_https {
        //         // HTTPS setup
        //         #[cfg(feature = "ssl")]
        //         {
        //             let ssl_key = config_clone.ssl_key.expect("SSL key file is required for HTTPS");
        //             let ssl_cert = config_clone.ssl_cert.expect("SSL cert file is required for HTTPS");
        //
        //             // Here would be your SSL setup code
        //             // Placeholder for SSL code
        //         }
        //
        //         // Placeholder for the actual HTTPS server setup
        //         println!("HTTPS server started on {}", addr);
        //         Ok(())
        //     } else {
        //         // HTTP setup
        //         let make_svc = hyper::service::make_service_fn(|_conn| async {
        //             Ok::<_, hyper::Error>(hyper::service::service_fn(|_req| async {
        //                 Ok::<_, hyper::Error>(hyper::Response::new(hyper::Body::from("Hello World!")))
        //             }))
        //         });
        //
        //         let server = hyper::server::Server::bind(&addr).serve(make_svc);
        //         println!("HTTP server started on {}", addr);
        //         server.await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Hyper error: {}", e)))
        //     }
        // })

pub trait DummyContextTrait: Send + Sync {}

#[derive(Debug, Clone, Copy)]
pub struct DummyContext {}

impl DummyContextTrait for DummyContext {}
