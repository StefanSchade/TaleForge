use std::path::Path;
use std::sync::Arc;

use actix_web::web::Data;

use adapter::web::app_state::AppState;
use adapter::web::server;
use port::dto::player_state_dto::PlayerStateDTO;

use crate::service_container::service_container::ServiceContainer;

mod data_loader;
pub mod service_container;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // initialize dependency injection container

    let container = ServiceContainer::new();

    // initialize the repos with data
    let location_file_path = Path::new("resources_test/locations.json");
    let passage_file_path = Path::new("resources_test/passages.json");

    data_loader::load_data_from_json(
        container.repo().location(),
        container.repo().passage(),
        &location_file_path,
        &passage_file_path,
    )?;

    // initialize player 1
    container.repo().player_state()
        .save(
            PlayerStateDTO {
                player_id: 1,
                current_location_id: 1,
            }
        );

    let app_state = Data::new(Arc::new(AppState::new(
        container.repo().location(),
        container.repo().passage(),
        container.repo().player_state(),
        container.domain_story().move_player(),
    )));

    if let Err(e) = server::start_server(app_state).await {
        eprintln!("Server failed to start: {}", e);
    }

    Ok(())
}