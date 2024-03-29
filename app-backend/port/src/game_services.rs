use crate::use_case::move_player::{MovePlayerCommand, MovePlayerResult};

pub trait GameService {
    fn move_player(&self, input: MovePlayerCommand) -> Result<MovePlayerResult, &'static str>;
}