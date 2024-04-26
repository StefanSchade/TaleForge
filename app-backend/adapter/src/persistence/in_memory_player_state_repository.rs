use futures::future::BoxFuture;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use crosscutting::error_management::error::Error;
use port::dto::player_state_dto::PlayerStateDTO;
use port::repositories::player_state_repository::PlayerStateRepository;

#[derive(Clone)]
pub struct InMemoryPlayerStateRepository {
    states: Arc<Mutex<HashMap<i32, PlayerStateDTO>>>,
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
        f.debug_struct("InMemoryPlayerStateRepository")
            .finish()
    }
}

impl PlayerStateRepository for InMemoryPlayerStateRepository {
    fn find_by_player_id(&self, id: i32) -> BoxFuture<'static, Result<Option<PlayerStateDTO>, Error>> {
        let states = self.states.clone();
        Box::pin(async move {
            let states = states.lock().await;
            Ok(states.get(&id).cloned())
        })
    }

    fn save(&self, player_state: PlayerStateDTO) -> BoxFuture<'static, Result<Option<PlayerStateDTO>, Error>> {
        let states = self.states.clone();
        Box::pin(async move {
            let mut states = states.lock().await;
            println!("Updating InMemoryRepositoryPlayerState with {:?}", &player_state);
            states.insert(player_state.player_id, player_state.clone());
            Ok(Some(player_state))
        })
    }
}
