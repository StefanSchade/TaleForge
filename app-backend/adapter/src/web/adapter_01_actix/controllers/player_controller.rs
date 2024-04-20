use std::sync::Arc;

use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

use port::context::RequestContext;
use port::domain_stories::move_player::MovePlayerCommand;

use crate::web::adapter_01_actix::app_state::AppState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebMovePlayerInput {
    pub direction: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebMovePlayerOutput {
    pub location: i32,
    pub narration: String,
}

pub async fn move_player(data: web::Data<Arc<AppState>>, web_input: web::Json<WebMovePlayerInput>) -> impl Responder {
    let extracted_player_id = 1;

    let command = MovePlayerCommand::from(web_input.into_inner());
    let context = RequestContext::new(Some(extracted_player_id));
    let result = data.move_player_domain_story.execute(context, command);

    match result {
        Ok(response) => {
            let web_output = WebMovePlayerOutput {
                location: response.player_location,
                narration: response.narration,
            };
            HttpResponse::Ok().json(web_output)
        }
        Err(error) => HttpResponse::InternalServerError().body(error),
    }
}

impl From<WebMovePlayerInput> for MovePlayerCommand {
    fn from(input: WebMovePlayerInput) -> Self {
        MovePlayerCommand { direction: input.direction }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use actix_web::{App, test, web};

    use port::domain_stories::move_player::MockMovePlayerDomainStory;
    use port::dto::location_dto::LocationDTO;
    use port::dto::passage_dto::PassageDTO;
    use port::dto::player_state_dto::PlayerStateDTO;
    use port::repositories::location_repository::MockLocationRepository;
    use port::repositories::passage_repository::MockPassageRepository;
    use port::repositories::player_state_repository::MockPlayerStateRepository;

    use super::*;

    #[actix_rt::test]
    async fn test_player_move() {
        let mock_location = LocationDTO {
            id: 1,
            description: "test description".to_string(),
            title: "test title".to_string(),
            image_url: None,
        };

        let mock_location_repository = Arc::new(MockLocationRepository::new(mock_location, None));

        let mock_passage = PassageDTO {
            id: 1,
            from_location_id: 1,
            to_location_id: 2,
            direction: "north".to_string(),
            description: "desc".to_string(),
            narration: "nar".to_string(),
        };
        let mock_passage_repository = Arc::new(MockPassageRepository::new(mock_passage, None));

        let mock_player_state = PlayerStateDTO {
            player_id: 1,
            current_location_id: 1,
        };
        let mock_player_state_repository = Arc::new(MockPlayerStateRepository::new(mock_player_state));

        let mock_move_player_domain_story = Arc::new(MockMovePlayerDomainStory::new(1, "You moved north.".to_string()));

        let app_state = web::Data::new(Arc::new(AppState::new(
            mock_location_repository,
            mock_passage_repository,
            mock_player_state_repository,
            mock_move_player_domain_story,
        )));

        let app = test::init_service(
            App::new()
                .app_data(app_state)
                .configure(crate::web::adapter_01_actix::server::configure_routes) // Ensure correct function use
        ).await;

        // Create an instance of WebMovePlayerInput and serialize it
        let test_input = WebMovePlayerInput {
            direction: "north".to_string(),
        };
        let request_body = serde_json::to_string(&test_input).expect("Failed to serialize test input");

        let req = test::TestRequest::post()
            .uri("/player/move")
            .set_payload(request_body)
            .insert_header(("Content-Type", "application/json"))  // Ensure the content type is set
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success(), "Expected success, got {:?}", resp.status());

        let status = resp.status();

        // Deserialize the response body to WebMovePlayerOutput
        let result: WebMovePlayerOutput = test::read_body_json(resp).await;

        println!("Response Body: {:?}", result.clone() );

        // Now use the previously captured status
        println!("Response Status: {:?}", status);

        assert_eq!(result.location, 1, "Expected location to be 1");
        assert_eq!(result.narration, "You moved north.", "Expected narration to match");
    }
}