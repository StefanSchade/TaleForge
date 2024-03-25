// src/port/dto.rs

// Input DTO for moving a player

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MovePlayerCommand {
    pub direction: String,
}
#[derive(Serialize, Deserialize)]
pub struct MovePlayerResult {
    pub player_location: i32,
    pub narration: String,
}
