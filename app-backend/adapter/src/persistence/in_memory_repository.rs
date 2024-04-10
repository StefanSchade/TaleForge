use std::collections::HashMap;
use std::option::Option;
use std::sync::Mutex;
use port::dto::location_dto::LocationDTO;
use port::dto::passage_dto::PassageDTO;
use port::dto::player_state_dto::PlayerStateDTO;

use port::repositories::location_repository::LocationRepository;
use port::repositories::passage_repository::PassageRepository;
use port::repositories::player_state_repository::PlayerStateRepository;

pub struct InMemoryLocationRepository {
    locations: Mutex<HashMap<i32, LocationDTO>>,
}


pub struct InMemoryPassageRepository {
    passages: Mutex<HashMap<i32, PassageDTO>>,
}


pub struct InMemoryPlayerStateRepository {
    states: Mutex<HashMap<i32, PlayerStateDTO>>,
}


impl InMemoryLocationRepository {
    pub fn new() -> Self {
        // Initialize with test data or leave empty to be filled dynamically
        Self {
            locations: Mutex::new(HashMap::new()),
        }
    }
}

impl LocationRepository for InMemoryLocationRepository {

    fn get_location_by_id(&self, id: i32) -> Option<LocationDTO> {
        let locations = self.locations.lock().unwrap(); // Acquire the lock
        locations.get(&id).cloned() // Now we can call .get() on the HashMap
    }

    fn get_all_locations(&self) -> Vec<LocationDTO> {
        let locations = self.locations.lock().expect("Mutex lock failed");
        locations.values().cloned().collect()
    }

    fn add_location(&self, location: LocationDTO) -> Result<(), String> {
        let mut locations = self.locations.lock().map_err(|_| "Mutex lock failed")?;
        locations.insert(location.id, location);
        Ok(())
    }
}

impl InMemoryPassageRepository {
    pub fn new() -> Self {
        // Initialize with test data or leave empty to be filled dynamically
        Self {
            passages: Mutex::new(HashMap::new()),
        }
    }
}


impl PassageRepository for InMemoryPassageRepository {
    fn get_passage_by_id(&self, id: i32) -> Option<PassageDTO> {
        let passages_lock = self.passages.lock().unwrap(); // Acquire the lock
        passages_lock.get(&id).cloned() // Perform the get operation
    }

    fn get_passages_for_location(&self, location_id: i32) -> Vec<PassageDTO> {
        let passages_lock = self.passages.lock().unwrap(); // Acquire the lock
        passages_lock.values()
            .filter(|passage| passage.from_location_id == location_id || passage.to_location_id == location_id)
            .cloned()
            .collect() // Collect filtered and cloned passages into a Vec
    }
    fn find_passage_by_location_and_direction(&self, location_id: i32, direction: &str) -> Option<PassageDTO> {
        let passages = self.passages.lock().unwrap();
        passages.values().find(|&passage|
            (passage.from_location_id == location_id) && (passage.direction.eq_ignore_ascii_case(direction))
        ).cloned()
    }

    fn add_passage(&self, passage: PassageDTO) -> Result<(), String> {
        let mut locations = self.passages.lock().map_err(|_| "Mutex lock failed")?;
        locations.insert(passage.id, passage);
        Ok(())
    }

    fn find_by_start_and_end_id(&self, from_location_id: i32, to_location_id: i32) -> Option<PassageDTO> {
        self.passages.lock().unwrap().values()
            .find(|passage| passage.from_location_id == from_location_id && passage.to_location_id == to_location_id)
            .cloned()
    }
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