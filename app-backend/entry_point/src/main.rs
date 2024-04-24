use std::path::Path;
use std::sync::Arc;

use adapter::persistence::{
    in_memory_location_repository::InMemoryLocationRepository,
    in_memory_passage_repository::InMemoryPassageRepository,
    in_memory_player_state_repository::InMemoryPlayerStateRepository,
};
use adapter::web::option_01_actixweb::server::ActixWebServer;
use application::domain_story_impl::move_player_impl::MovePlayerDomainStoryImpl;
use port::dto::player_state_dto::PlayerStateDTO;
use port::service_container::service_container::ServiceContainer;
use env_logger::Env;

mod data_loader;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let location_repo = Arc::new(InMemoryLocationRepository::new());
    let passage_repo = Arc::new(InMemoryPassageRepository::new());
    let player_state_repo = Arc::new(InMemoryPlayerStateRepository::new());

    let location_file_path = Path::new("resources_test/locations.json");
    let passage_file_path = Path::new("resources_test/passages.json");

    let data = data_loader::load_data_from_json(
        location_repo.clone(),
        passage_repo.clone(),
        &location_file_path,
        &passage_file_path
    ).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)));

    match data {
        Ok(_) => println!("Data loaded successfully."),
        Err(e) => {
            eprintln!("Failed to load data: {:?}", e);
            return Err(e);
        }
    }

    let move_player_ds = Arc::new(MovePlayerDomainStoryImpl::new(
        location_repo.clone(),
        passage_repo.clone(),
        player_state_repo.clone(),
    ));

    let service_container = ServiceContainer::new(move_player_ds);

    let server = ActixWebServer::new(service_container);
    server.start_server().await
}