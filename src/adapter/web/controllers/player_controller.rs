use std::sync::Arc;

use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;

pub async fn move_player(data: web::Data<Arc<AppState>>, input: web::Json<MovePlayerInput>) -> impl Responder {
    let result = data.move_player_use_case.execute(input.direction.clone());
    match result {
        Ok(response) => HttpResponse::Ok().json(response), // Adjust based on your actual response structure
        Err(error) => HttpResponse::InternalServerError().body(error),
    }
}

#[derive(Serialize, Deserialize)]
pub struct MovePlayerInput {
    pub direction: String,
}