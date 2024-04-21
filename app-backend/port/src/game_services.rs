use crate::port_services::domain_story_move_player::{MovePlayerCommand, MovePlayerResult};

pub trait GameService {
    fn move_player(&self, input: MovePlayerCommand) -> Result<MovePlayerResult, &'static str>;
}