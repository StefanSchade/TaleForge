use std::sync::Arc;

use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::port::dto::MovePlayerCommand;

#[derive(Deserialize)]
pub struct WebMovePlayerInput {
    pub direction: String,
}

#[derive(Serialize)]
pub struct WebMovePlayerOutput {
    pub location: i32,
    pub narration: String,
}

pub async fn move_player(data: web::Data<Arc<AppState>>, web_input: web::Json<WebMovePlayerInput>) -> impl Responder {
    let command = MovePlayerCommand::from(web_input.into_inner());
    let result = data.move_player_use_case.execute(command);

    match result {
        Ok(response) => {
            let web_output = WebMovePlayerOutput {
                location: response.player_location,
                narration: response.narration,
            };
            HttpResponse::Ok().json(web_output)
        }
        Err(error) => HttpResponse::InternalServerError().body(error),
    }
}

impl From<WebMovePlayerInput> for MovePlayerCommand {
    fn from(input: WebMovePlayerInput) -> Self {
        MovePlayerCommand { direction: input.direction }
    }
}

