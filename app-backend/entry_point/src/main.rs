use std::path::Path;
use std::sync::Arc;

use adapter::persistence::in_memory_location_repository::InMemoryLocationRepository;
use adapter::persistence::in_memory_passage_repository::InMemoryPassageRepository;
use adapter::persistence::in_memory_player_state_repository::InMemoryPlayerStateRepository;
use adapter::web::option_01_actixweb::server::ActixWebServer;
use application::domain_story_impl::move_player_impl::MovePlayerDomainStoryImpl;
use port::dto::player_state_dto::PlayerStateDTO;
use port::repositories::player_state_repository::PlayerStateRepository;
use port::service_container::service_container::ServiceContainer;
use env_logger::Env;
use application::contract_implementations::location_query_impl::LocationQueryImpl;
use application::contract_implementations::passage_query_impl::PassageQueryImpl;
use application::domain_story_impl::move_player_impl2::MovePlayerDomainStoryImpl2;
use crosscutting::error_management::error::Error;
use domain_contract::services::navigation_services::NavigationService;


mod data_loader;

fn assert_send<T: Send>() {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    assert_send::<MovePlayerDomainStoryImpl>();
    assert_send::<InMemoryPlayerStateRepository>();
    assert_send::<InMemoryLocationRepository>();
    assert_send::<InMemoryLocationRepository>();
    assert_send::<NavigationService>();
    assert_send::<Error>();
    assert_send::<LocationQueryImpl>();
    assert_send::<PassageQueryImpl>();

    env_logger::init_from_env(Env::default().default_filter_or("info"));  // Adjust log level as needed

    // instantiate the outbound adapters ...

    let location_repo = Arc::new(InMemoryLocationRepository::new());
    let passage_repo = Arc::new(InMemoryPassageRepository::new());
    let player_state_repo = Arc::new(InMemoryPlayerStateRepository::new());

    // ... and initialize them

    let location_file_path = Path::new("resources_test/locations.json");
    let passage_file_path = Path::new("resources_test/passages.json");


    // If `load_data_from_json` returns a known error type that already implements `Send + Sync`, use that.
// Otherwise, wrap the error into `std::io::Error` like this:

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
            return Err(e); // Return the error to stop further execution
        }
    }

    let _= player_state_repo
        .save(
            PlayerStateDTO {
                player_id: 1,
                current_location_id: 1,
            }
        );

    // This is externalized because the adapter has no access to the domain and the constructor

    let move_player_ds = Arc::new(MovePlayerDomainStoryImpl2::new(
        location_repo.clone(),
        passage_repo.clone(),
        player_state_repo.clone(),
    ));

    // hand everything to a container

    let service_container = ServiceContainer::new(
        move_player_ds,
    );


    let server = ActixWebServer::new(service_container);
    server.start_server().await
}