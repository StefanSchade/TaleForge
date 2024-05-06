use std::path::Path;
use std::sync::Arc;

use env_logger::Env;
use log::info;
use adapter::persistence::in_memory_bout_repository::InMemoryBoutRepository;

use adapter::persistence::in_memory_location_repository::InMemoryLocationRepository;
use adapter::persistence::in_memory_passage_repository::InMemoryPassageRepository;
use adapter::persistence::in_memory_player_state_repository::InMemoryPlayerStateRepository;
use adapter::web::option_01_actixweb::actix_web_server::ActixWebServer;
use application::domain_story_impl::move_player_domain_story_impl::MovePlayerDomainStoryImpl;
use port::dto::player_state_dto::PlayerStateDTO;
use port::repositories::player_state_repository::PlayerStateRepository;
use port::adapters_inbound::web_server::WebServer;
use port::adapters_outbound::service_container::ServiceContainer;

mod data_loader;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));  // Adjust log level as needed

    // instantiate the outbound adapters ...

    let location_repo = Arc::new(InMemoryLocationRepository::new());
    let passage_repo = Arc::new(InMemoryPassageRepository::new());
    let bout_repo = Arc::new(InMemoryBoutRepository::new());
    let player_state_repo = Arc::new(InMemoryPlayerStateRepository::new());

    // // ... and initialize them
    //
    // let _ = player_state_repo
    //     .save(
    //         PlayerStateDTO {
    //             player_id: 1,
    //             bout_id: 1,
    //             current_location_id: 1,
    //         }
    //     ).await;
    //
    // info!("player state initialized");

    let location_file_path = Path::new("resources_test/locations.json");
    let passage_file_path = Path::new("resources_test/passages.json");
    let bout_file_path = Path::new("resources_test/bout.json");

    // If `load_data_from_json` returns a known error type that already implements `Send + Sync`, use that.
    // Otherwise, wrap the error into `std::io::Error` like this:

    let data = data_loader::load_data_from_json(
        location_repo.clone(),
        passage_repo.clone(),
        bout_repo.clone(),
        &location_file_path,
        &passage_file_path,
        &bout_file_path,
    ).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)));


    match data {
        Ok(_) => println!("Data loaded successfully."),
        Err(e) => {
            eprintln!("Failed to load data: {:?}", e);
            return Err(e); // Return the error to stop further execution
        }
    }

    let move_player_ds = Arc::new(MovePlayerDomainStoryImpl::new(
        location_repo.clone(),
        passage_repo.clone(),
        bout_repo.clone(),
        player_state_repo.clone()
    ));

    let service_container = ServiceContainer::new(move_player_ds);

    // hyper adapter

    let service_container_hyper = service_container.clone();

    let adapter: Adapter = Adapter::new(service_container_hyper);

    // actix adapter

    let actix = ActixWebServer::new(service_container);

    actix.start_server().await
}