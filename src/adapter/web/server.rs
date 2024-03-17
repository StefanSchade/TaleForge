// src/server.rs

use std::sync::Arc;

use actix_web::{App, HttpServer, web};

use crate::adapter::web::controllers::player_controller;
use crate::app_state::AppState;

pub async fn start_server(app_state: web::Data<Arc<AppState>>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(
                web::resource("/player/move").route(web::post().to(player_controller::move_player)),
            )
        // Add more routes and controllers as needed
    })
        .bind("localhost:8080")?
        .run()
        .await
}
