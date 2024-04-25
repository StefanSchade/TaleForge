use futures::future::BoxFuture;
use crate::context::RequestContext;
use std::fmt::Debug;
use crate::port_services::domain_story_move_player::{MovePlayerCommand, MovePlayerResult};

pub trait MovePlayerDomainStory2: Send + Sync + Debug {
    fn execute(&self, context: RequestContext, input: MovePlayerCommand) -> BoxFuture<'static, Result<MovePlayerResult, String>> ;
}
