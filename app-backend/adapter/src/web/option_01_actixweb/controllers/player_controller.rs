use std::sync::Arc;

use actix_web::{HttpResponse, Responder, web};
use log::{debug, info};
use serde::{Deserialize, Serialize};

use port::port_services::domain_story_move_player::MovePlayerDomainStoryRequest;

use crate::web::option_01_actixweb::app_state::AppState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebMovePlayerInput {
    pub direction: String,
    pub bout_id: i64,
    pub player_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebMovePlayerOutput {
    pub location: i64,
    pub narration: String,
}

pub async fn move_player(
    app_state: web::Data<Arc<AppState>>,
    web_input: web::Json<WebMovePlayerInput>,
) -> impl Responder {
    info!("Handling move_player request: {:?}", web_input);
    debug!("Attempting to extract AppState...");

    let command = MovePlayerDomainStoryRequest::from(web_input.into_inner());

    let domain_story = app_state.move_player_domain_story.clone();

    let result = domain_story.execute(command).await;

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


impl From<WebMovePlayerInput> for MovePlayerDomainStoryRequest {
    fn from(input: WebMovePlayerInput) -> Self {
        MovePlayerDomainStoryRequest
        {
            direction: input.direction,
            bout_id: input.bout_id,
            player_id: input.player_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use actix_web::{App, test, web};

    use port::dto::location_dto::LocationDTO;
    use port::dto::passage_dto::PassageDTO;
    use port::port_services::domain_story_move_player::MockMovePlayerDomainStory;
    use port::repositories::location_repository::MockLocationRepository;
    use port::repositories::passage_repository::MockPassageRepository;

    use super::*;

    #[actix_rt::test]
    async fn test_player_move() {
        let mock_location = LocationDTO {
            id: 1,
            game_id: 1,
            description: "test description".to_string(),
            title: "test title".to_string(),
            image_url: None,
        };

        let _mock_location_repository = Arc::new(MockLocationRepository::new(mock_location, None));

        let mock_passage = PassageDTO {
            id: 1,
            game_id: 2,
            from_location_id: 1,
            to_location_id: 2,
            direction: "north".to_string(),
            description: "desc".to_string(),
            narration: "nar".to_string(),
        };
        let _mock_passage_repository = Arc::new(MockPassageRepository::new(mock_passage, None));

        let mock_move_player_domain_story = Arc::new(MockMovePlayerDomainStory::new(1, "You moved north.".to_string()));

        let app_state = web::Data::new(Arc::new(AppState::new(
            mock_move_player_domain_story
        )));

        let app = test::init_service(
            App::new()
                .app_data(app_state)
                .configure(crate::web::option_01_actixweb::actix_web_server::ActixWebServer::configure_routes) // Ensure correct function use
        ).await;

        // Create an instance of WebMovePlayerInput and serialize it
        let test_input = WebMovePlayerInput {
            direction: "north".to_string(),
            bout_id: 1,
            player_id: 1,
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

        println!("Response Body: {:?}", result.clone());

        // Now use the previously captured status
        println!("Response Status: {:?}", status);

        assert_eq!(result.location, 1, "Expected location to be 1");
        assert_eq!(result.narration, "You moved north.", "Expected narration to match");
    }
}