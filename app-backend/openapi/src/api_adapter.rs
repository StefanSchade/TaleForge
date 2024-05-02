use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use std::task::{Context, Poll};
use async_trait::async_trait;
use frunk_core::labelled::chars::C;
use swagger::ApiError;
use MovePlayerPostResponse::PlayerMovedSuccessfully;
use crate::{Api, models, MovePlayerPostResponse};
use crate::models::MovePlayerPostRequest;

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
    async fn move_player(&self, bout_id: u64, player_id: u64, direction: String) -> Result<String, String>;
}

struct ApiAdapter<B: BusinessAdapter + Send + Sync> {
    business_adapter: B,
}

#[async_trait]
impl<B: BusinessAdapter + Send + Sync> Api<RequestContext> for ApiAdapter<B> {
    // fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
    //     Poll::Ready(Ok(()))
    // }

    async fn move_player_post(&self, move_player_post_request: models::MovePlayerPostRequest, context: &RequestContext) -> Result<MovePlayerPostResponse, ApiError> {
        let bout_id = match u64::from_str(&move_player_post_request.bout_id) {
            Ok(num) => num,
            Err(_) => return Err(ApiError::invalid_input("Invalid bout_id")),
        };

        let player_id = match u64::from_str(&move_player_post_request.player_id) {
            Ok(num) => num,
            Err(_) => return Err(ApiError::invalid_input("Invalid player_id")),
        };

        let direction = move_player_post_request.direction;

        match self.business_adapter.move_player(bout_id, player_id, direction, context.token.clone()).await {
            Ok(result_json) => Ok(MovePlayerPostResponse::PlayerMovedSuccessfully(result_json)),
            Err(error_message) => Err(ApiError::from(error_message)),
        }
    }
}