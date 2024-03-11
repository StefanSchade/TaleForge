use std::sync::Arc;
use actix_web::{web, App, HttpServer, HttpResponse};
use crate::application::use_cases::move_player::{MovePlayer, MovePlayerUseCase};
use crate::port::dto::MovePlayerInput;

pub async fn start_server() -> std::io::Result<()> {
    // Wrap the use case in Arc for shared ownership and thread safety
    let move_player_use_case = Arc::new(MovePlayerUseCase {});

    HttpServer::new(move || {
        let move_player_use_case = move_player_use_case.clone(); // Clone Arc for use inside the async move block

        App::new()
            .service(
                web::resource("/move")
                    .route(web::post().to(move |input: web::Json<MovePlayerInput>| {
                        let use_case = move_player_use_case.clone(); // Clone Arc for use inside the async block
                        async move {
                            let result = use_case.move_player(input.into_inner());
                            match result {
                                Ok(output) => HttpResponse::Ok().json(output),
                                Err(error) => HttpResponse::BadRequest().body(error),
                            }
                        }
                    })),
            )
    })
        .bind("localhost:8080")?
        .run()
        .await
}