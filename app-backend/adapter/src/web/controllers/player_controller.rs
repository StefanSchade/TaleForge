use std::sync::Arc;

use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

use port::context::RequestContext;
use port::domain_stories::move_player::MovePlayerCommand;

use crate::web::app_state::AppState;

#[derive(Deserialize)]
pub struct WebMovePlayerInput {
    pub direction: String,
}

#[derive(Serialize)]
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
    use actix_web::{HttpMessage, HttpRequest, test};
    use actix_web::http::header::ContentType;
    use actix_web::web::Data;
    use actix_web::App;
    use port::domain_stories::move_player::{MockMovePlayerDomainStory, MovePlayerCommand, MovePlayerResult};
    use port::dto::location_dto::LocationDTO;
    use port::dto::passage_dto::PassageDTO;
    use port::dto::player_state_dto::PlayerStateDTO;
    use port::repositories::location_repository::MockLocationRepository;
    use port::repositories::passage_repository::MockPassageRepository;
    use port::repositories::player_state_repository::MockPlayerStateRepository;

    use crate::web::app_state::AppState;
    use crate::web::server;

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

        let mock_move_player_domain_story = Arc::new(MockMovePlayerDomainStory::new(1, "narration".to_string()));

        let app_state = AppState::new(
            mock_location_repository,
            mock_passage_repository,
            mock_player_state_repository,
            mock_move_player_domain_story,
        );

       // let app = server::start_server(Data::new(Arc::new(app_state))).await;

        let app = test::init_service(App::new()).await;


        let req_body = serde_json::to_string(&MovePlayerCommand {
            direction: "north".into(),
        }).expect("Failed to serialize request");

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .uri("/player/move")
            .set_payload(req_body)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let result: MovePlayerResult = test::read_body_json(resp).await;
        assert_eq!(result.player_location, 1);
        assert_eq!(result.narration, "You moved north.");
    }
}