use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use log::info;

use port::service_container::service_container::ServiceContainer;

use crate::web::option_01_actixweb::app_state::AppState;
use crate::web::option_01_actixweb::controllers::{player_controller};

#[derive(Clone)]
pub struct SimpleState;


pub struct ActixWebServer {
    pub service_container: Arc<ServiceContainer>,
}

impl ActixWebServer {
    pub fn start_server(&self) -> Pin<Box<dyn Future<Output=Result<(), std::io::Error>> + Send>> {
        info!("starting actix");

        let app_state = Data::new(AppState {
            move_player_domain_story: Arc::clone(&self.service_container.move_player()),
        });

        info!("Configured App Data for Actix: {:?}", &self.service_container.move_player());

        let server_future = async move {
            let server = HttpServer::new(move || {
                App::new()
                    .app_data(app_state.clone())
                    .configure(Self::configure_routes)
            })
                .bind("localhost:8080")?;
            server.run().await
        };

        info!("actix has been started");

        Box::pin(server_future)
    }

    fn configure_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/player/move").route(web::post().to(player_controller::move_player))
        );
    }

    pub fn new(container: ServiceContainer) -> Arc<Self> {
        Arc::new(ActixWebServer {
            service_container: Arc::new(container),
        })
    }
}