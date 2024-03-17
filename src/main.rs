use std::path::Path;
use std::sync::Arc;

use actix_web::web::Data;

use adapter::web::server;
use app_state::AppState;

use crate::adapter::persistence::in_memory_repository::{InMemoryLocationRepository, InMemoryPassageRepository};
use crate::application::use_cases::move_player::MovePlayerUseCase;
use crate::domain::navigation_services::NavigationService;
use crate::port::repository::{LocationRepository, PassageRepository};

// src/main.rs
mod data_loader;
//mod domain;
mod port;
mod adapter;
mod application;
mod domain;
mod app_state;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    // setup the repositories
    let location_repository: Arc<dyn LocationRepository> = Arc::new(InMemoryLocationRepository::new());
    let passage_repository: Arc<dyn PassageRepository> = Arc::new(InMemoryPassageRepository::new());

    let app_state = Data::new(Arc::new(AppState::new(
        location_repository.clone(),
        passage_repository.clone(),
    )));

    // initialize the repos with data
    let location_file_path = Path::new("resources_test/locations.json");
    let passage_file_path = Path::new("resources_test/passages.json");

    data_loader::load_data_from_json(
        location_repository.clone(), // Assuming your function expects Arc<R> where R: LocationRepository
        passage_repository.clone(), // Assuming your function expects Arc<P> where P: PassageRepository
        &location_file_path,
        &passage_file_path,
    )?;

    let navigation_service = NavigationService::new(location_repository.clone(), passage_repository.clone());

    let move_player_use_case = MovePlayerUseCase::new(
        location_repository.clone(),
        passage_repository.clone(),
        navigation_service,
    );

    server::start_server(app_state).await;

    Ok(())
}