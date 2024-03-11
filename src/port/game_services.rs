use crate::port::dto::{MovePlayerInput, MovePlayerOutput};

pub trait GameService {
    fn move_player(&self, input: MovePlayerInput) -> Result<MovePlayerOutput, &'static str>;
}