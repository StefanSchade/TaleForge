use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use std::task::{Context, Poll};
use async_trait::async_trait;
use frunk_core::labelled::chars::C;
use swagger::ApiError;
use MovePlayerPostResponse::PlayerMovedSuccessfully;
use crate::{Api, models, MovePlayerPostResponse, MovePlayerResponse};
use crate::models::{MovePlayerPostRequest, MovePlayerRequest};

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub token: String,
}

impl RequestContext {
    pub fn new(token: &str) -> Self {
        RequestContext {
            token: token.to_string(),
        }
    }
}

pub trait BusinessAdapter: Send + Sync {
    async fn move_player(&self, bout_id: u64, player_id: u64, direction: String, string: String) -> Result<String, String>;
}

struct ApiAdapter<B: BusinessAdapter + Send + Sync> {
    business_adapter: B,
}

#[async_trait]
impl<B: BusinessAdapter + Send + Sync> Api<RequestContext> for ApiAdapter<B> {
    // fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
    //     Poll::Ready(Ok(()))
    // }

    async fn move_player(&self, move_player_request: MovePlayerRequest, context: &RequestContext) -> Result<MovePlayerResponse, ApiError> {

        let bout_id = move_player_request.bout_id;
        let player_id = move_player_request.player_id;
        let direction = move_player_request.direction;

        if bout_id < 0 || player_id < 0 {
            return Err(ApiError("No negative id allowed".to_string()));
        }

        let bout_id = bout_id as u64;
        let player_id = player_id as u64;

        match self.business_adapter.move_player(bout_id, player_id, direction, context.token.clone()).await {
            Ok(result_json) => Ok(MovePlayerResponse::PlayerMovedSuccessfully(result_json)),
            Err(error_message) => Err(ApiError::from(error_message)),
        }
    }
}