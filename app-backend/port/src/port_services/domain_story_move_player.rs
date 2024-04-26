use async_trait::async_trait;
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use crate::context::RequestContext;
use std::fmt::Debug;
use std::future;
use futures::FutureExt;

#[async_trait]
pub trait MovePlayerDomainStory: Send + Sync + Debug {
    fn execute(&self, context: RequestContext, input: MovePlayerCommand) -> BoxFuture<'static, Result<MovePlayerResult, String>> ;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MovePlayerCommand {
    pub direction: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MovePlayerResult {
    pub player_location: i32,
    pub narration: String,
}

#[cfg(feature = "test-utils")]
#[derive(Debug)]
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
#[async_trait]
impl MovePlayerDomainStory for MockMovePlayerDomainStory {
    fn execute(&self, _context: RequestContext, _input: MovePlayerCommand) -> BoxFuture<'static, Result<MovePlayerResult, String>> {
        future::ready(Ok(self.fixed_result.clone())).boxed()
    }
}
