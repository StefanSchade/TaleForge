use std::future::Future;
use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use actix_web::web::Data;

use port::service_container::service_container::ServiceContainer;

use crate::web::option_01_actixweb::app_state::AppState;
use crate::web::option_01_actixweb::controllers::player_controller;
use crate::web::webserver_interface::WebServer;

pub struct ActixWebServer {
    pub service_container: Arc<ServiceContainer>,
}

impl WebServer for ActixWebServer {
    fn start_server(&self) -> impl Future<Output=Result<(), std::io::Error>> + Send {
        let app_state = Data::new(AppState {
            move_player_domain_story: Arc::clone(&self.service_container.domain_story().move_player()),
        });

        let server = HttpServer::new(move || {
            App::new()
                .app_data(app_state.clone())
                .configure(configure_routes)
        });

        // The server is a future that needs to be boxed to be returned
        Box::pin(async move {
            server.bind("localhost:8080")?.run().await
        })
    }

    fn new(container: ServiceContainer) -> Arc<Self> where Self: Sized + Sync + Send {
        Arc::new(ActixWebServer {
            service_container: Arc::new(container),
        })
    }
}

// Function to configure routes
fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/player/move").route(web::post().to(player_controller::move_player))
    );
    // Additional routes can be added here
}
