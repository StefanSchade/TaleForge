use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use actix_web::web::Data;

use port::service_container::service_container::ServiceContainer;

use crate::web::option_01_actixweb::app_state::AppState;
use crate::web::option_01_actixweb::controllers::player_controller;


use std::future::Future;
use std::pin::Pin;


pub struct ActixWebServer {
    pub service_container: Arc<ServiceContainer>,
}
impl ActixWebServer {
    pub fn start_server(&self) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send>> {
        let app_state = Data::new(AppState {
            location_repository: Arc::clone(&self.service_container.outbound_adapters().location_repo()),
            passage_repository: Arc::clone(&self.service_container.outbound_adapters().passage_repo()),
            player_state_repository: Arc::clone(&self.service_container.outbound_adapters().player_state_repo()),
            move_player_domain_story: Arc::clone(&self.service_container.domain_story().move_player()),
        });

        let server_future = async move {
            let server = HttpServer::new(move || {
                App::new()
                    .app_data(app_state.clone())
                    .configure(Self::configure_routes)
            })
                .bind("localhost:8080")?;

            server.run().await
        };

        Box::pin(server_future)
    }

    fn configure_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/player/move").route(web::post().to(player_controller::move_player))
        );
        // Additional routes can be added here
    }

    pub fn new(container: ServiceContainer) -> Arc<Self> {
        Arc::new(ActixWebServer {
            service_container: Arc::new(container),
        })
    }
}