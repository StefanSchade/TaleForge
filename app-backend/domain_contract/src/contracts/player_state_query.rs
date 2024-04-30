use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

use crosscutting::error_management::error::Error;
use domain_pure::model::player_state::PlayerState;

pub trait PlayerStateQuery: Send + Sync + Debug {
    fn get_player_state(&self, player_id: u64, bout_id: u64) -> Pin<Box<dyn Future<Output=Result<Option<PlayerState>, Error>> + Send + 'static>>;
    fn persist_player_state(&self, player_state: PlayerState, bout_id: u64) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'static>>;
}