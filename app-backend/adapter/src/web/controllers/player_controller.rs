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
    use actix_web::{App, http, HttpResponse, test, web};
    use actix_web::dev::Service;
    use actix_web::web::Data;
    use serde::{Deserialize, Serialize};
    use port::dto::location_dto::LocationDTO;

    //   use port::domain_stories::move_player::{MockMovePlayerDomainStory, MovePlayerCommand, MovePlayerResult};
    use port::repositories::location_repository::{MockLocationRepository};
    //use port::repositories::passage_repository::MockPassageRepository;
    //use port::repositories::player_state_repository::MockPlayerStateRepository;

    use crate::web::app_state::AppState;
    use crate::web::server;

    use super::*;

    #[actix_rt::test]
    async fn test_player_move() {

        // let mock_location = LocationDTO{
        //     id: 1,
        //     description: "test description",
        //     title: "test title",
        //     image_url: None
        // };
        //
        // let mock_repo = MockLocationRepository::new(vec![test_location]);
        // let mock_repo_shared = Arc::new(Mutex::new(mock_repo));
        //

        //let mock_passage_repo = Arc::new(MockPassageRepository::new());
        // let mock_player_state_repo = Arc::new(MockPlayerStateRepository::new());
        // let mock_move_player_domain_story = Arc::new(MockMovePlayerDomainStory::new());



        // let app_state = AppState::new(
        //     mock_location_repo,
        //     mock_passage_repo,
        //     mock_player_state_repo,
        //     mock_move_player_domain_story,
        // );

       // let app = server::start_server(Data::new(Arc::new(app_state))).await;

        // let req_body = serde_json::to_string(&MovePlayerCommand {
        //     direction: "north".into(),
        // }).expect("Failed to serialize request");
        //
        // let req = test::TestRequest::post()
        //     .uri("/player/move")
        //     .header("Content-Type", "application/json")
        //     .set_payload(req_body)
        //     .to_request();

        // let resp = test::call_service(&app, req).await;
        // assert!(resp.status().is_success());
        //
        // let result: MovePlayerResult = test::read_body_json(resp).await;
        // assert_eq!(result.player_location, 1);
        // assert_eq!(result.narration, "You moved north.");
    }
}