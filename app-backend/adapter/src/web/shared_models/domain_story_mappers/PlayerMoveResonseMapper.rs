use crosscutting::error_management::error::Error;
use crosscutting::error_management::standard_errors::{ID_MUST_BE_POSITIVE_INT, MANDATORY_FIELD_MISSING};
use openapi_client::models::{MovePlayerRequest, MovePlayerResponse};
use port::port_services::domain_story_move_player::{MovePlayerDomainStoryRequest, MovePlayerDomainStoryResponse};

use crate::web::shared_models::MapperTrait::MapperTrait;

struct PlayerMoveResponse {}

impl MapperTrait<MovePlayerRequest, MovePlayerDomainStoryRequest> for PlayerMoveResponse {
    fn from_api(api_model: MovePlayerRequest) -> Result<MovePlayerDomainStoryRequest, Error> {
        let player_id = api_model.player_id.ok_or_else(
            || MANDATORY_FIELD_MISSING.instantiate(
                vec![
                    "MovePlayerDomainStoryRequest".to_string(),
                    "player_id".to_string(),
                ]))?;
        let bout_id = api_model.bout_id.ok_or_else(
            || MANDATORY_FIELD_MISSING.instantiate(
                vec![
                    "MovePlayerDomainStoryRequest".to_string(),
                    "bout_id".to_string(),
                ]))?;
        let direction = api_model.direction.ok_or_else(
            || MANDATORY_FIELD_MISSING.instantiate(
                vec![
                    "MovePlayerDomainStoryRequest".to_string(),
                    "direction".to_string(),
                ]))?;

        Ok(MovePlayerDomainStoryRequest {
            direction,
            bout_id,
            player_id,
        })
    }

    fn to_api(port_model: MovePlayerDomainStoryResponse) -> Result<MovePlayerResponse, Error> {


        if port_model.player_location < 0 {
            return Err(ID_MUST_BE_POSITIVE_INT.instantiate(
                vec![
                    "domain_model.player_location".to_string(),
                    format!("{:?}", port_model.player_location),
                ]));
        }

        if port_model.narration.is_empty() {
            return Err(MANDATORY_FIELD_MISSING.instantiate(
                vec![
                    "MovePlayerDomainStoryResponse".to_string(),
                    "domain_model.narration".to_string(),
                ]));
        }

        Ok(
            MovePlayerResponse {
                player_location: Option::from(port_model.player_location),
                narration: Option::from(port_model.narration),
            }
        )
    }
}
