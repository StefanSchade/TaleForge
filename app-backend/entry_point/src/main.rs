use std::path::Path;
use std::sync::Arc;

use adapter::persistence::in_memory_repository::{InMemoryLocationRepository, InMemoryPassageRepository, InMemoryPlayerStateRepository};
use adapter::web::adapter_01_actix::server;
use application::domain_story_impl::move_player_impl::MovePlayerDomainStoryImpl;
use port::dto::player_state_dto::PlayerStateDTO;
use port::service_container::service_container::ServiceContainer;

mod data_loader;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let location_repo = Arc::new(InMemoryLocationRepository::new());
    let passage_repo = Arc::new(InMemoryPassageRepository::new());
    let player_state_repo = Arc::new(InMemoryPlayerStateRepository::new());

    // let location_queries = Arc::new(LocationQueryImpl::new(location_repo));
    // let passage_queries =  Arc::new(PassageQueryImpl::new(passage_repo));

    let move_player_ds = Arc::new(MovePlayerDomainStoryImpl::new(
        location_repo.clone(),
        passage_repo.clone(),
        player_state_repo.clone(),
    ));

    let container = ServiceContainer::new(
        location_repo,
        passage_repo,
        player_state_repo,
        move_player_ds,
    );

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

    if let Err(e) = server::start_server(container).await {
        eprintln!("Server failed to start: {}", e);
    }

    Ok(())
}