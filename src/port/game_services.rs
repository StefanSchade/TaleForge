use crate::port::dto::{MovePlayerCommand, MovePlayerResult};

pub trait GameService {
    fn move_player(&self, input: MovePlayerCommand) -> Result<MovePlayerResult, &'static str>;
}