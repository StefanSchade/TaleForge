use std::fmt::Debug;

use async_trait::async_trait;
use futures::future::BoxFuture;
use futures::FutureExt;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait MovePlayerDomainStory: Send + Sync + Debug {
    fn execute(&self, input: MovePlayerDomainStoryRequest) -> BoxFuture<'static, Result<MovePlayerDomainStoryResponse, String>>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MovePlayerDomainStoryRequest {
    pub direction: String,
    pub bout_id: i64,
    pub player_id: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MovePlayerDomainStoryResponse {
    pub player_location: i64,
    pub narration: String,
}

#[cfg(feature = "test-utils")]
#[derive(Debug)]
pub struct MockMovePlayerDomainStory {
    pub fixed_result: MovePlayerDomainStoryResponse,
}

#[cfg(feature = "test-utils")]
impl MockMovePlayerDomainStory {
    pub fn new(player_location: i64, narration: String) -> Self {
        MockMovePlayerDomainStory {
            fixed_result: MovePlayerDomainStoryResponse {
                player_location,
                narration,
            }
        }
    }
}

#[cfg(feature = "test-utils")]
#[async_trait]
impl MovePlayerDomainStory for MockMovePlayerDomainStory {
    fn execute(&self, _input: MovePlayerDomainStoryRequest) -> BoxFuture<'static, Result<MovePlayerDomainStoryResponse, String>> {
        std::future::ready(Ok(self.fixed_result.clone())).boxed()
    }
}