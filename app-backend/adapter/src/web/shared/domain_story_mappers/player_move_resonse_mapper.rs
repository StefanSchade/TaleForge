use log::info;
use crosscutting::error_management::error::Error;
use crosscutting::error_management::error_kind::ErrorKind;
use crosscutting::error_management::standard_errors::{ID_MUST_BE_POSITIVE_INT, MANDATORY_FIELD_MISSING};
#[allow(unused_imports)]
use openapi_client::models::Error as ErrorBody;
#[allow(unused_imports)]
use openapi_client::models::MovePlayerRequest as ModelMovePlayerRequest;
#[allow(unused_imports)]
use openapi_client::models::MovePlayerResponse as MovePlayerResponseBody;
#[allow(unused_imports)]
use openapi_client::MovePlayerResponse as MovePlayerResponseCodesAndBody;
use port::port_services::domain_story_move_player::MovePlayerDomainStoryResponse;

use crate::web::shared::response_mapper_trait::ResponseMapperTrait;

pub struct PlayerMoveResponseMapper {}
impl ResponseMapperTrait<MovePlayerResponseCodesAndBody, MovePlayerResponseBody, MovePlayerDomainStoryResponse> for PlayerMoveResponseMapper {
    fn to_api_body(port_model: MovePlayerDomainStoryResponse) -> Result<MovePlayerResponseBody, Error> {
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

        Ok(MovePlayerResponseBody {
            new_location: Some(port_model.player_location),
            narration: Some(port_model.narration),
        })
    }

    fn to_api_200(port_model: MovePlayerDomainStoryResponse) -> MovePlayerResponseCodesAndBody {
        info!("to_api_response 1");
        match Self::to_api_body(port_model) {
            Ok(body) => { info!("to_api_response 2"); MovePlayerResponseCodesAndBody::PlayerMovedSuccessfully(body) },
            Err(error) => {
                info!("move_player error ({:?}) ", error);

                let error_payload = ErrorBody::new(error.code.to_string(), error.message.to_string());
                match error.kind {
                    ErrorKind::Functional => {
                        match error.code.as_str() {
                            "mandatory_field_missing" | "id_must_be_positive_int" => MovePlayerResponseCodesAndBody::InvalidInput(error_payload),
                            "player_not_registered" | "bout_not_running" => MovePlayerResponseCodesAndBody::PlayerOrBoutNotFound(error_payload),
                            _ => MovePlayerResponseCodesAndBody::InvalidInput(error_payload), // Default functional error mapping
                        }
                    },
                    ErrorKind::Technical => {
                        println!("Technical Error Occurred: {}", error); // Logging the error
                        MovePlayerResponseCodesAndBody::InternalServerError(ErrorBody::new(
                            "internal_server_error".to_string(),
                            "An unexpected internal server error occurred.".to_string(),
                        ))
                    }
                }
            }
        }
    }

    fn to_api_error_codes(port_model: MovePlayerDomainStoryResponse) -> MovePlayerResponseCodesAndBody {
        info!("to_api_response 1");
        match Self::to_api_body(port_model) {
            Ok(body) => { info!("to_api_response 2"); MovePlayerResponseCodesAndBody::PlayerMovedSuccessfully(body) },
            Err(error) => {
                info!("move_player error ({:?}) ", error);

                let error_payload = ErrorBody::new(error.code.to_string(), error.message.to_string());
                match error.kind {
                    ErrorKind::Functional => {
                        match error.code.as_str() {
                            "mandatory_field_missing" | "id_must_be_positive_int" => MovePlayerResponseCodesAndBody::InvalidInput(error_payload),
                            "player_not_registered" | "bout_not_running" => MovePlayerResponseCodesAndBody::PlayerOrBoutNotFound(error_payload),
                            _ => MovePlayerResponseCodesAndBody::InvalidInput(error_payload), // Default functional error mapping
                        }
                    },
                    ErrorKind::Technical => {
                        println!("Technical Error Occurred: {}", error); // Logging the error
                        MovePlayerResponseCodesAndBody::InternalServerError(ErrorBody::new(
                            "internal_server_error".to_string(),
                            "An unexpected internal server error occurred.".to_string(),
                        ))
                    }
                }
            }
        }
    }


}
