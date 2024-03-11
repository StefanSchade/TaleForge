use actix_web::{web, App, HttpServer};

pub async fn start_server() {
    HttpServer::new(|| {
        App::new()
            .route("/move", web::post().to(move_player_endpoint))
        // more routes here
    })
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
        .unwrap();
}