use std::sync::Arc;


use port::use_cases::move_player::MovePlayerUseCase;
use port::repository::{LocationRepository, PassageRepository, PlayerStateRepository};

pub struct AppState {
    pub location_repository: Arc<dyn LocationRepository>,
    pub passage_repository: Arc<dyn PassageRepository>,
    pub player_state_repository:  Arc<dyn PlayerStateRepository>,
    pub move_player_use_case: Arc<dyn MovePlayerUseCase>,
}

impl AppState {
    pub fn new(location_repository: Arc<dyn LocationRepository>, passage_repository: Arc<dyn PassageRepository>, player_state_repository: Arc<dyn PlayerStateRepository>, move_player_use_case: Arc<dyn MovePlayerUseCase>) -> Self {
        AppState { location_repository, passage_repository, player_state_repository, move_player_use_case }
    }
}