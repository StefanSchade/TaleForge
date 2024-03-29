
use serde::{Deserialize, Serialize};
use crate::context::RequestContext;



pub trait MovePlayerUseCase: Send + Sync {
    fn execute(&self,context: RequestContext, command: MovePlayerCommand) -> Result<MovePlayerResult, String>;
}

#[derive(Serialize, Deserialize)]
pub struct MovePlayerCommand {
    pub direction: String,
}
#[derive(Serialize, Deserialize)]
pub struct MovePlayerResult {
    pub player_location: i32,
    pub narration: String,
}
