use actix_web::{web, App, HttpServer, web::ServiceConfig};
use std::sync::Arc;

use crate::web::controllers::player_controller;
use crate::web::app_state::AppState;

// Adjust the function to work with ServiceConfig
pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/player/move").route(web::post().to(player_controller::move_player))
    );
    // Add more routes here as needed
}

// Function to start the server
pub async fn start_server(app_state: web::Data<Arc<AppState>>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(configure_routes)  // Use the new configure_routes function
    })
        .bind("localhost:8080")?
        .run()
        .await
}