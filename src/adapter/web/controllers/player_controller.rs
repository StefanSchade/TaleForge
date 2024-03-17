// src/controllers/player_controller.rs

use std::sync::Arc;

use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;

pub async fn move_player(data: web::Data<Arc<AppState>>, input: web::Json<MovePlayerInput>) -> impl Responder {
    // Example handler logic
    HttpResponse::Ok().json("This is a response from the move_player endpoint")
}

#[derive(Serialize, Deserialize)]
pub struct MovePlayerInput {
    pub direction: String,
}