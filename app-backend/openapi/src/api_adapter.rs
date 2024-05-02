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

struct ApiAdapter {
    business_adapter: BusinessAdapter
}

#[async_trait]
impl Api for ApiAdapter {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        todo!()
    }

    async fn move_player_post(&self, move_player_post_request: MovePlayerPostRequest, context: &C) -> Result<MovePlayerPostResponse, ApiError> {
        todo!()
    }
}