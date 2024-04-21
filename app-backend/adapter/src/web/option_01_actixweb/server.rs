use std::sync::Arc;

use actix_web::{App, HttpServer, web, web::ServiceConfig};
use actix_web::web::Data;

use port::service_container::service_container::ServiceContainer;

use crate::web::option_01_actixweb::app_state::AppState;
use crate::web::option_01_actixweb::controllers::player_controller;
use crate::web::webserver_interface::WebServer;

pub struct ActixWebServer {
    pub port: Arc<ServiceContainer>,
}

//#[async_trait]
impl WebServer for ActixWebServer {
    async fn start_server(&self) -> Result<(), std::io::Error> {
        let app_state = Data::new(Arc::new(AppState::new(
            self.port.outbound_adapters().location_repo(),
            self.port.outbound_adapters().passage_repo(),
            self.port.outbound_adapters().player_state_repo(),
            self.port.port_services().move_player(),
        )));

        let server = HttpServer::new(move || {
            App::new()
                .app_data(app_state.clone())
                .configure(configure_routes)
        });

        server.bind("localhost:8080")?.run().await
    }
    fn new(container: ServiceContainer) -> Arc<Self> where Self: Sized + Sync + Send {
        Arc::new(ActixWebServer {
            port: Arc::new(container),
        })
    }
}


// Function to configure routes
pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/player/move").route(web::post().to(player_controller::move_player))
    );
    // Additional routes can be added here
}
