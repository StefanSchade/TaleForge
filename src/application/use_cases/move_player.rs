use crate::port::dto::{MovePlayerInput, MovePlayerOutput};

pub trait MovePlayer {
    fn move_player(&self, input: MovePlayerInput) -> Result<MovePlayerOutput, &'static str>;
}

#[derive(Clone)]
pub struct MovePlayerUseCase;

impl MovePlayer for MovePlayerUseCase {
    fn move_player(&self, _input: MovePlayerInput) -> Result<MovePlayerOutput, &'static str> {
        // Placeholder implementation
        Ok(MovePlayerOutput {
            room_number: 1,
            title: "The Enchanted Forest".to_string(),
            description: "You are standing at the edge of an enchanted forest. Trees whisper in a language forgotten by time.".to_string(),
            image_url: Some("http://example.com/enchanted_forest.jpg".to_string()),
        })
    }
}