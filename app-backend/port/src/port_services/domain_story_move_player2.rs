use async_trait::async_trait;
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use crate::context::RequestContext;
use std::fmt::Debug;
use crate::port_services::domain_story_move_player::{MovePlayerCommand, MovePlayerResult};

#[async_trait]
pub trait MovePlayerDomainStory2: Send + Sync + Debug {
    fn execute(&self, context: RequestContext, input: MovePlayerCommand) -> BoxFuture<'static, Result<MovePlayerResult, String>> ;
}
