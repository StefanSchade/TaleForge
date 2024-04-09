use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use actix_web::web::Data;

use crate::web::app_state::AppState;
use crate::web::controllers::player_controller;

pub async fn start_server(app_state: Data<Arc<AppState>>) -> std::io::Result<()> {
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
