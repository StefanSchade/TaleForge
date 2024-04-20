use actix_web::{web, App, HttpServer, web::ServiceConfig};
use std::sync::Arc;
use actix_web::web::Data;
use port::service_container::service_container::ServiceContainer;

use crate::web::adapter_01_actix::controllers::player_controller;
use crate::web::adapter_01_actix::app_state::AppState;

// Adjust the function to work with ServiceConfig
pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/player/move").route(web::post().to(player_controller::move_player))
    );
    // Add more routes here as needed
}

// Function to start the server
pub async fn start_server(container: ServiceContainer) -> std::io::Result<()> {

    let app_state = Data::new(Arc::new(AppState::new(
         container.repo().location(),
         container.repo().passage(),
         container.repo().player_state(),
         container.domain_story().move_player(),
    )));

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(configure_routes)  // Use the new configure_routes function
    })
        .bind("localhost:8080")?
        .run()
        .await
}