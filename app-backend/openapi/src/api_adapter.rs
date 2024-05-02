use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use std::task::{Context, Poll};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use swagger::ApiError;
use crate::{Api, models, MovePlayerResponse};
use crate::models::{MovePlayer200Response, MovePlayerRequest};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovePlayerOpenApiResult {
    pub player_location: u64,
    pub narration: String,
}


impl TryFrom<MovePlayerOpenApiResult> for MovePlayer200Response {
    type Error = &'static str;

    fn try_from(value: MovePlayerOpenApiResult) -> Result<Self, Self::Error> {
        let player_location = i64::try_from(value.player_location)
            .map_err(|_| "Value out of range for i64")?;

        Ok(MovePlayer200Response {
            player_location: Some(player_location),
            narration: Some(value.narration),
        })
    }
}


pub trait BusinessAdapter: Send + Sync {
    async fn move_player(&self, bout_id: u64, player_id: u64, direction: String) -> Result<MovePlayerOpenApiResult, String>;
}

struct ApiAdapter<B: BusinessAdapter + Send + Sync> {
    business_adapter: Arc<B>,
}


impl<B: BusinessAdapter + Send + Sync> ApiAdapter<B> {
    pub fn new(business_adapter: Arc<B>) -> Self {
        ApiAdapter { business_adapter }
    }
}

#[async_trait]
impl<B: BusinessAdapter + Send + Sync> Api<RequestContext> for ApiAdapter<B> {

    async fn move_player(&self, move_player_request: MovePlayerRequest, context: &RequestContext) -> Result<MovePlayerResponse, ApiError> {

        let bout_id = move_player_request.bout_id;
        let player_id = move_player_request.player_id;
        let direction = move_player_request.direction;

        if bout_id < 0 || player_id < 0 {
            return Err(ApiError("No negative id allowed".to_string()));
        }

        let bout_id = bout_id as u64;
        let player_id = player_id as u64;

        match self.business_adapter.move_player(bout_id, player_id, direction).await {
            Ok(result) => {
                let response_dto = MovePlayer200Response::try_from(result)
                    .map_err(|_| ApiError("Failed to convert result".into()))?;
                Ok(MovePlayerResponse::PlayerMovedSuccessfully(response_dto))
            },
            Err(error_message) => Err(ApiError(error_message)),
        }
    }
}