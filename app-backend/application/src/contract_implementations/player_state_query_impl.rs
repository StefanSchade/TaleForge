use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use crosscutting::error_management::error::Error;
use domain_contract::contracts::player_state_query::PlayerStateQuery;
use domain_pure::model::player_state::PlayerState;
use port::repositories::player_state_repository::PlayerStateRepository;

#[derive(Clone, Debug)]
pub struct  PlayerStateQueryImpl {
    player_state_repository: Arc<dyn PlayerStateRepository>,
}

impl PlayerStateQueryImpl {
    pub fn new (player_state_repository: Arc<dyn PlayerStateRepository>) -> Self {
        PlayerStateQueryImpl {player_state_repository}
    }
}

impl PlayerStateQuery for PlayerStateQueryImpl {
    fn get_player_state(&self, player_id: u64, bout_id: u64) -> Pin<Box<dyn Future<Output=Result<Option<PlayerState>, Error>> + Send + 'static>> {
        todo!()
    }

    fn save_newly_initialized_player(&self, player_state: PlayerState, bout_id: u64) -> Pin<Box<dyn Future<Output=Result<Option<PlayerState>, Error>> + Send + 'static>> {
        todo!()
    }



}