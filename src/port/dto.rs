// src/port/dto.rs

// Input DTO for moving a player
#[derive(serde::Deserialize)]
pub struct MovePlayerInput {
    pub direction: String,
}

// Output DTO for the move player response
#[derive(serde::Serialize)]
pub struct MovePlayerOutput {
    pub room_number: i32,
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
}