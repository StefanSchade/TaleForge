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
use crosscutting::error_management::error::Error;
use domain_contract::services::navigation_services::NavigationService;
use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use adapter::web::option_01_actixweb::app_state::AppState;
use adapter::web::option_01_actixweb::controllers::player_controller;

mod data_loader;

#[tokio::main]
async fn main() -> std::io::Result<()> {

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

    let move_player_ds = Arc::new(MovePlayerDomainStoryImpl::new(
        location_repo.clone(),
        passage_repo.clone(),
        player_state_repo.clone(),
    ));

    let app_state = AppState {
        move_player_domain_story: move_player_ds,
    };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Arc::new(app_state.clone())))
            .service(
                web::resource("/player/move").route(web::post().to(player_controller::move_player))
            )
        // Add other services and configuration as needed
    })
        .bind("localhost:8080")?
        .run()
        .await

}