use std::sync::Arc;


use port::domain_stories::move_player::MovePlayerDomainStory;
use port::repositories::location_repository::LocationRepository;
use port::repositories::passage_repository::PassageRepository;
use port::repositories::player_state_repository::PlayerStateRepository;

pub struct AppState {
    pub location_repository: Arc<dyn LocationRepository>,
    pub passage_repository: Arc<dyn PassageRepository>,
    pub player_state_repository:  Arc<dyn PlayerStateRepository>,
    pub move_player_domain_story:   Arc<dyn  MovePlayerDomainStory>,
}

impl AppState {
    pub fn new(location_repository: Arc<dyn LocationRepository>, passage_repository: Arc<dyn PassageRepository>, player_state_repository: Arc<dyn PlayerStateRepository>, move_player_use_case:  Arc<dyn  MovePlayerDomainStory>) -> Self {
        AppState { location_repository, passage_repository, player_state_repository, move_player_domain_story: move_player_use_case }
    }
}