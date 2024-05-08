use std::path::Path;
use std::sync::Arc;
use clap::{App, Arg};

use env_logger::Env;

use adapter::persistence::in_memory_bout_repository::InMemoryBoutRepository;
use adapter::persistence::in_memory_location_repository::InMemoryLocationRepository;
use adapter::persistence::in_memory_passage_repository::InMemoryPassageRepository;
use adapter::persistence::in_memory_player_state_repository::InMemoryPlayerStateRepository;
use adapter::web::option_01_actixweb::actix_web_server::ActixWebServer;
use adapter::web::option_02_hyper::hyper_server::HyperServer;
use application::domain_story_impl::move_player_domain_story_impl::MovePlayerDomainStoryImpl;
use port::adapters_inbound::web_server::{ServerConfig, WebServer};
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
        player_state_repo.clone(),
    ));

    let service_container = ServiceContainer::new(move_player_ds);

    // hyper adapter

    let service_container_hyper = service_container.clone();

    let matches = App::new("server")
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .get_matches();

    let addr = "127.0.0.1:8080";

    let hyper_future =  adapter::web::option_02_hyper::hyper_server::create(addr, matches.is_present("https"), service_container_hyper);

    let actix = ActixWebServer::new(service_container, ServerConfig::new(
        "localhost:8081".to_string(),
        false,
        None,
        None,
    ));

    let actix_future = actix.start_server();

    let (_hyper_result, _actix_result) = tokio::join!(hyper_future, actix_future);

    Ok(())
}