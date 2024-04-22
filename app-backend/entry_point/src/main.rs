use std::path::Path;
use std::sync::Arc;

use adapter::persistence::in_memory_location_repository::InMemoryLocationRepository;
use adapter::persistence::in_memory_passage_repository::InMemoryPassageRepository;
use adapter::persistence::in_memory_player_state_repository::InMemoryPlayerStateRepository;
use adapter::web::option_01_actixweb::server::ActixWebServer;
use adapter::web::webserver_interface::WebServer;
use application::domain_story_impl::move_player_impl::MovePlayerDomainStoryImpl;
use port::dto::player_state_dto::PlayerStateDTO;
use port::repositories::player_state_repository::PlayerStateRepository;
use port::service_container::service_container::ServiceContainer;

mod data_loader;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // instantiate the outbound adapters ...

    let location_repo = Arc::new(InMemoryLocationRepository::new());
    let passage_repo = Arc::new(InMemoryPassageRepository::new());
    let player_state_repo = Arc::new(InMemoryPlayerStateRepository::new());

    // ... and initialize them

    let location_file_path = Path::new("resources_test/locations.json");
    let passage_file_path = Path::new("resources_test/passages.json");

    data_loader::load_data_from_json(
        location_repo.clone(),
        passage_repo.clone(),
        &location_file_path,
        &passage_file_path,
    ).await.unwrap();

    player_state_repo
        .save(
            PlayerStateDTO {
                player_id: 1,
                current_location_id: 1,
            }
        );

    // This is externalized because the adapter has no access to the domain and the constructor

    let move_player_ds = Arc::new(MovePlayerDomainStoryImpl::new(
        location_repo.clone(),
        passage_repo.clone(),
        player_state_repo.clone(),
    ));

    // hand everything to a container

    let service_container = ServiceContainer::new(
        location_repo,
        passage_repo,
        player_state_repo,
        move_player_ds,
    );

    // start server

    let server = ActixWebServer::new(service_container);
    server.start_server().await?;

    Ok(())
}