// use std::sync::Arc;
// use async_trait::async_trait;
// use openapi_client::{Api, models, MovePlayerPostResponse};
// use port::port_services::domain_story_move_player::MovePlayerDomainStory;
//
//
//
// struct ApiAdapter {
//     player_movement_service: Arc<dyn MovePlayerDomainStory>,
// }
//
// #[async_trait]
// impl Api for ApiAdapter {
//     async fn move_player_post(
//         &self,
//         move_player_post_request: models::MovePlayerPostRequest,
//         context: &C
//     ) -> Result<MovePlayerPostResponse, ApiError> {
//         let result = self.player_movement_service.move_player(
//             move_player_post_request.bout_id,
//             move_player_post_request.player_id,
//             move_player_post_request.direction
//         ).await;
//
//         match result {
//             Ok(move_result) => Ok(MovePlayerPostResponse::Successful),
//             Err(e) => Err(ApiError::from(e)),
//         }
//     }
// }
