use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::io::Error;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use hyper::server::conn::Http;
use log::info;
use swagger::{ApiError, EmptyContext, Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use tokio::net::TcpListener;

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

pub async fn create(addr: &str, https: bool, container: ServiceContainer) {
    let addr = addr.parse().expect("Failed to parse bind address");
    let server = HyperServer::new(container);
    let service = MakeService::new(server);
    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
        let mut service =
        openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Clone)]
pub struct HyperServer<C> {
    service_container: ServiceContainer,
    marker: PhantomData<C>,
}


impl<C> HyperServer<C> {
    pub fn new(container: ServiceContainer) -> Self {
        HyperServer {
            service_container: container,
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
                info!("request({:?}) - X-Span-ID: {:?}", request, context.get().0.clone());

                let domain_response_result = domain_story.execute(request).await;
                print!("got here 3");
                let response = domain_response_result.map_err(|e| {
                    print!("got here 1!!!! THIS IS THE PROBEM");
                    ApiError(format!("Error processing domain story: {}", e))
                })?;
                print!("got here 2");
                Ok(PlayerMoveResponseMapper::to_api_response_codes(response))
            }
            Err(e) => {
                Err(ApiError(format!("Error processing domain story: {}", e)))
            }
        }
    }
    //Err(e) => {   Err(ApiError(format ! ("Error processing move player request: {}", e)))   }
}