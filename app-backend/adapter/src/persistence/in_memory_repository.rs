use std::collections::HashMap;
use std::option::Option;
use std::sync::Mutex;
use port::dto::location_dto::LocationDTO;
use port::dto::passage_dto::PassageDTO;
use port::dto::player_state_dto::PlayerStateDTO;

use port::repositories::location_repository::LocationRepository;
use port::repositories::passage_repository::PassageRepository;
use port::repositories::player_state_repository::PlayerStateRepository;



pub struct InMemoryPlayerStateRepository {
    states: Mutex<HashMap<i32, PlayerStateDTO>>,
}



impl InMemoryPlayerStateRepository {
    pub fn new() -> Self {
        InMemoryPlayerStateRepository {
            states: Mutex::new(HashMap::new()),
        }
    }
}


impl PlayerStateRepository for InMemoryPlayerStateRepository {
    fn find_by_player_id(&self, id: i32) -> Option<PlayerStateDTO> {
        let states = self.states.lock().unwrap();
        states.get(&id).cloned()
    }

    fn save(&self, player_state: PlayerStateDTO) {
        let mut states = self.states.lock().unwrap();
        println!("Updating InMemoryRepositoryPlayerState with {:?}", &player_state);
        states.insert(player_state.player_id, player_state);
    }
}