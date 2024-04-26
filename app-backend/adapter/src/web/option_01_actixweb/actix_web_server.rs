use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use log::info;
use port::service_container::service_container::ServiceContainer;
use crate::web::option_01_actixweb::app_state::AppState;
use crate::web::option_01_actixweb::controllers::player_controller;


pub struct ActixWebServer {
    pub service_container: ServiceContainer,
}

impl ActixWebServer {
    // pub fn start_server(sc: ServiceContainer) -> Pin<Box<dyn Future<Output=Result<(), std::io::Error>> + Send>> {
    pub fn start_server(&self) -> Pin<Box<dyn Future<Output=Result<(), std::io::Error>> + Send>> {
        info!("starting actix");

        let app_state = AppState {
            move_player_domain_story: self.service_container.move_player(),
        };

        let server_future = async move {
            let server = HttpServer::new(move || {
                App::new()
                    .app_data(Data::new(Arc::new(app_state.clone())))
                    .service(
                        web::resource("/player/move").route(web::post().to(player_controller::move_player))
                    )
                // Add other services and configuration as needed
            })
                .bind("localhost:8080")?;
            server.run()
                .await
        };

        Box::pin(server_future)
    }
}


impl ActixWebServer {
    pub fn new(container: ServiceContainer) -> Arc<Self> {
        Arc::new(ActixWebServer {
            service_container: container,
        })
    }
}