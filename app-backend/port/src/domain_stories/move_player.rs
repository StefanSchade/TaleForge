use serde::{Deserialize, Serialize};
use crate::context::RequestContext;

pub trait MovePlayerDomainStory: Send + Sync {
    fn execute(&self,context: RequestContext, command: MovePlayerCommand) -> Result<MovePlayerResult, String>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MovePlayerCommand {
    pub direction: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct MovePlayerResult {
    pub player_location: i32,
    pub narration: String,
}

#[cfg(feature = "test-utils")]
pub struct MockMovePlayerDomainStory {
    pub fixed_result: MovePlayerResult,
}
#[cfg(feature = "test-utils")]
impl MockMovePlayerDomainStory {
    pub fn new(player_location: i32, narration: String) -> Self {
        MockMovePlayerDomainStory{
            fixed_result: MovePlayerResult{
                player_location,
                narration
            }
        }
    }
}

#[cfg(feature = "test-utils")]
impl MovePlayerDomainStory for MockMovePlayerDomainStory {
    fn execute(&self, _context: RequestContext, _command: MovePlayerCommand) -> Result<MovePlayerResult, String> {
        Ok(self.fixed_result.clone())
    }
}
