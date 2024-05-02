// src/generated_server/api_adapter.rs

use std::error::Error;
use std::task::{Context, Poll};
use async_trait::async_trait;
use swagger::ApiError;
use crate::{Api, MovePlayerPostResponse};
use crate::models::MovePlayerPostRequest;

pub trait BusinessAdapter {
    async fn move_player(&self, bout_id: u64, player_id: u64, direction: String) ->Result<String, String>;
}


struct ApiAdapter<B: BusinessAdapter + Send + Sync> {
    business_adapter: B,
}

#[async_trait]
impl<B: BusinessAdapter + Send + Sync> Api for ApiAdapter<B> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    async fn move_player_post(&self, move_player_post_request: MovePlayerPostRequest, _context: &C) -> Result<MovePlayerPostResponse, ApiError> {
        let bout_id = move_player_post_request.bout_id;
        let player_id = move_player_post_request.player_id;
        let direction = move_player_post_request.direction;

        match self.business_adapter.move_player(bout_id, player_id, direction).await {
            Ok(result_json) => {
                Ok(MovePlayerPostResponse::Status200(MovePlayerPost200Response { result: result_json }))
            },
            Err(error_message) => {
                Ok(MovePlayerPostResponse::Status400(ErrorModel { message: error_message }))
            }
        }
    }
}