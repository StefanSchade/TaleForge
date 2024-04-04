mod data_loader;

use std::path::Path;
use std::sync::Arc;

use actix_web::web::Data;

use adapter::web::server;
use adapter::web::app_state::AppState;

use adapter::persistence::in_memory_repository::{InMemoryLocationRepository, InMemoryPassageRepository, InMemoryPlayerStateRepository};
use application::domain_story_impl::move_player_impl::MovePlayerUseCaseImpl;
use domain_pure::model::player_state::PlayerState;
use domain_contract::services::navigation_services::{NavigationService, NavigationServiceTrait};


use port::repositories::location_repository::LocationRepository;
use port::repositories::passage_repository::PassageRepository;
use port::repositories::player_state_repository::PlayerStateRepository;


use application::contract_implementations::location_query_impl::LocationQueryImpl;
use application::contract_implementations::passage_query_impl::PassageQueryImpl;
use domain_contract::contracts::location_queries::navigation::LocationQueries;
use domain_contract::contracts::passage_queries::navigation::PassageQueries;
use port::domain_stories::move_player::MovePlayerUseCase;


// src/main.rs
#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    // setup the repositories
    let location_repository: Arc<dyn LocationRepository> = Arc::new(InMemoryLocationRepository::new());
    let passage_repository: Arc<dyn PassageRepository> = Arc::new(InMemoryPassageRepository::new());
    let player_state_repository: Arc<dyn PlayerStateRepository> = Arc::new(InMemoryPlayerStateRepository::new());

    // initialize the repos with data
    let location_file_path = Path::new("resources_test/locations.json");
    let passage_file_path = Path::new("resources_test/passages.json");
    // initialize player state for test user 1

    // let mut player_state_repository_mutable = player_state_repository.clone()

    player_state_repository.save(PlayerState::new(1,1));

    data_loader::load_data_from_json(
        location_repository.clone(), // Assuming your function expects Arc<R> where R: LocationRepository
        passage_repository.clone(), // Assuming your function expects Arc<P> where P: PassageRepository
        &location_file_path,
        &passage_file_path,
    )?;

    let location_queries : Arc<dyn LocationQueries> = Arc::new(LocationQueryImpl::new(location_repository.clone()));
    let passage_queries: Arc<dyn PassageQueries> = Arc::new(PassageQueryImpl::new(passage_repository.clone()));


    let navigation_service = NavigationService::new(location_queries.clone() , passage_queries.clone());


    let navigation_service_trait_object: Arc<dyn NavigationServiceTrait> = Arc::new(navigation_service);

    let move_player_use_case : Arc<dyn MovePlayerUseCase> = Arc::new( MovePlayerUseCaseImpl::new(
        location_repository.clone(),
        passage_repository.clone(),
        player_state_repository.clone(),
        navigation_service_trait_object.clone(),
    ));

    let app_state = Data::new(Arc::new(AppState::new(
        location_repository.clone(),
        passage_repository.clone(),
        player_state_repository.clone(),
        move_player_use_case.clone(),
    )));

    if let Err(e) = server::start_server(app_state).await {
        eprintln!("Server failed to start: {}", e);
    }

    Ok(())
}