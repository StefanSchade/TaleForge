use std::sync::Arc;

use crate::application::use_cases::move_player::MovePlayerUseCase;
use crate::port::repository::{LocationRepository, PassageRepository, PlayerStateRepository};

pub struct AppState {
    pub location_repository: Arc<dyn LocationRepository>,
    pub passage_repository: Arc<dyn PassageRepository>,
    pub player_state_repository:  Arc<dyn PlayerStateRepository>,
    pub move_player_use_case: MovePlayerUseCase,
}

impl AppState {
    pub fn new(location_repository: Arc<dyn LocationRepository>, passage_repository: Arc<dyn PassageRepository>, player_state_repository: Arc<dyn PlayerStateRepository>, move_player_use_case:  MovePlayerUseCase) -> Self {
        AppState { location_repository, passage_repository, player_state_repository, move_player_use_case }
    }
}