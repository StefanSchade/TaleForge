use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::Mutex;

use crosscutting::error_management::error::Error;
use port::dto::player_state_dto::PlayerStateDTO;
use port::repositories::player_state_repository::PlayerStateRepository;

#[derive(Clone)]
pub struct InMemoryPlayerStateRepository {
    states: Arc<Mutex<HashMap<u64, HashMap<u64, PlayerStateDTO>>>>,
}

impl InMemoryPlayerStateRepository {
    pub fn new() -> Self {
        InMemoryPlayerStateRepository {
            states: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl fmt::Debug for InMemoryPlayerStateRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InMemoryPlayerStateRepository").finish()
    }
}

impl PlayerStateRepository for InMemoryPlayerStateRepository {
    fn find_by_bout_id_and_player_id(&self, bout_id: u64, player_id: u64) -> BoxFuture<'static, Result<Option<PlayerStateDTO>, Error>> {
        let states = self.states.clone();
        Box::pin(async move {
            let states = states.lock().await;
            Ok(states.get(&bout_id)
                .and_then(|player_states| player_states.get(&player_id))
                .cloned())
        })
    }

    fn save(&self, player_state: PlayerStateDTO) -> BoxFuture<'static, Result<Option<PlayerStateDTO>, Error>> {
        let states = self.states.clone();
        Box::pin(async move {
            let mut states = states.lock().await;
            let player_states = states.entry(player_state.bout_id).or_insert_with(HashMap::new);
            println!("Updating InMemoryRepository PlayerState for bout_id {}, player_id {}, current location {}", player_state.bout_id, player_state.player_id, player_state.current_location_id);
            player_states.insert(player_state.player_id, player_state.clone());
            Ok(Some(player_state))
        })
    }
}
