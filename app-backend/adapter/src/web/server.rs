use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use actix_web::web::Data;

use crate::web::app_state::AppState;
use crate::web::controllers::player_controller;

pub async fn start_server(app_state: Data<Arc<AppState>>) -> std::io::Result<()> {
    HttpServer::new(move || {
        setup_app(app_state.clone())
    })
        .bind("localhost:8080")?
        .run()
        .await
}

pub fn setup_app(app_state: Data<Arc<AppState>>) -> App {
    App::new()
        .app_data(app_state.clone())
        .service(
            web::resource("/player/move").route(web::post().to(player_controller::move_player)),
        )
    // You can chain more configurations here as needed.
}
