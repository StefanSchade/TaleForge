use crosscutting::error_management::error::Error;
use crosscutting::error_management::standard_errors::{ID_MUST_BE_POSITIVE_INT, MANDATORY_FIELD_MISSING};
use openapi_client::models::MovePlayerRequest;
use port::port_services::domain_story_move_player::MovePlayerDomainStoryRequest;

use crate::web::shared::request_mapper_trait::RequestMapperTrait;

pub struct PlayerMoveRequestMapper {}

impl RequestMapperTrait<MovePlayerRequest, MovePlayerDomainStoryRequest> for PlayerMoveRequestMapper {
    fn from_api(api_model: MovePlayerRequest) -> Result<MovePlayerDomainStoryRequest, Error> {
        // Validate fields here
        if api_model.player_id < 0 {
            return Err(ID_MUST_BE_POSITIVE_INT.instantiate(
                vec!["MovePlayerRequest".to_string(), "player_id".to_string(), api_model.player_id.to_string()]));
        }

        if api_model.bout_id < 0 {
            return Err(ID_MUST_BE_POSITIVE_INT.instantiate(
                vec!["MovePlayerRequest".to_string(), "bout_id".to_string(), api_model.bout_id.to_string()]));
        }

        if api_model.direction.is_empty() {
            return Err(MANDATORY_FIELD_MISSING.instantiate(
                vec!["MovePlayerRequest".to_string(), "direction".to_string()]));
        }

        // If validation is successful, construct the domain model
        Ok(MovePlayerDomainStoryRequest {
            player_id: api_model.player_id,
            bout_id: api_model.bout_id,
            direction: api_model.direction,
        })
    }

    // Assume this remains unchanged
    fn to_api(port_model: MovePlayerDomainStoryRequest) -> Result<MovePlayerRequest, Error> {
        if port_model.player_id < 0 {
            return Err(ID_MUST_BE_POSITIVE_INT.instantiate(
                vec![
                    "domain_model.player_id".to_string(),
                    format!("{:?}", port_model.player_id),
                ]));
        }

        if port_model.bout_id < 0 {
            return Err(ID_MUST_BE_POSITIVE_INT.instantiate(
                vec![
                    "domain_model.bout_id".to_string(),
                    format!("{:?}", port_model.bout_id),
                ]));
        }

        if port_model.direction.is_empty() {
            return Err(MANDATORY_FIELD_MISSING.instantiate(
                vec![
                    "MovePlayerDomainStoryResponse".to_string(),
                    "domain_model.direction".to_string(),
                ]));
        }

        Ok(
            MovePlayerRequest {
                player_id: port_model.player_id,
                bout_id: port_model.bout_id,
                direction: port_model.direction,
            }
        )
    }
}