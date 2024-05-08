use std::future::Future;
use std::io::Error;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::{EmptyContext, Has, XSpanIdString};

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

use crate::web::option_02_hyper::app_state_hyper::AppStateHyper;

#[derive(Clone, Debug)]
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
            marker: PhantomData
        }
    }
}

impl WebServer for HyperServer<C> {
    fn start_server(&self) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
        let config_clone = self.config.clone();
        let app_state_hyper = AppStateHyper::new(self.service_container.move_player());

        Box::pin(async move {
            let addr = match config_clone.address.parse::<SocketAddr>() {
                Ok(addr) => addr,
                Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidInput, format!("Failed to parse bind address: {}", e))),
            };

            let service = MakeService::new(app_state_hyper);
            let service = MakeAllowAllAuthenticator::new(service, "cosmo");

            #[allow(unused_mut)]
                let mut service =
                openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(
                    service
                );

            if config_clone.use_https {
                #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
                {
                    unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
                }
                #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
                {
                    unimplemented!("For Linux see the example code in the generate");
            } else {
                    hyper::server::Server::bind(&addr).serve(service).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
            }
        }
    })
}
}

        // // Setup the server to use AppStateHyper for handling requests
        // let make_svc = make_service_fn(move |_conn| {
        //     let app_state_hyper_clone = app_state_hyper.clone(); // do we need this?
        //     async move {
        //         Ok::<_, hyper::Error>(service_fn(move |req| {
        //             app_state_hyper_clone.handle_request(req)
        //         }))
        //     }
        // });
        //
        // let server = hyper::server::Server::bind(&addr).serve(make_svc);
        // println!("Server running on http://{}", addr);
        // server.await.map_err(|e| Error::new(std::io::ErrorKind::Other, format!("Hyper server error: {}", e)))


pub trait DummyContextTrait: Send + Sync {}

#[derive(Debug, Clone, Copy)]
struct DummyContext {
    mut x_span_id: XSpanIdString
}

impl Has<XSpanIdString> for DummyContext {
    fn get(&self) -> &XSpanIdString {
        &self.x_span_id
    }

    fn get_mut(&mut self) -> &mut XSpanIdString {
        return &mut self.x_span_id;
    }

    fn set(&mut self, value: XSpanIdString) {
        &mut self.x_span_id = &mut value.clone();
    }
}