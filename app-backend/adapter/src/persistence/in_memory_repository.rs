use std::collections::HashMap;
use std::option::Option;
use std::sync::Mutex;

use domain_pure::model::location::Location;
use domain_pure::model::passage::Passage;
use domain_pure::model::player_state::PlayerState;

use port::repositories::location_repository::LocationRepository;
use port::repositories::passage_repository::PassageRepository;
use port::repositories::player_state_repository::PlayerStateRepository;

pub struct InMemoryLocationRepository {
    locations: Mutex<HashMap<i32, Location>>,
}


pub struct InMemoryPassageRepository {
    passages: Mutex<HashMap<i32, Passage>>,
}


pub struct InMemoryPlayerStateRepository {
    states: Mutex<HashMap<i32, PlayerState>>,
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

    fn get_location_by_id(&self, id: i32) -> Option<Location> {
        let locations = self.locations.lock().unwrap(); // Acquire the lock
        locations.get(&id).cloned() // Now we can call .get() on the HashMap
    }

    fn get_all_locations(&self) -> Vec<Location> {
        let locations = self.locations.lock().expect("Mutex lock failed");
        locations.values().cloned().collect()
    }

    fn add_location(&self, location: Location) -> Result<(), String> {
        let mut locations = self.locations.lock().map_err(|_| "Mutex lock failed")?;
        locations.insert(location.get_aggregate_id(), location);
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
    fn get_passage_by_id(&self, id: i32) -> Option<Passage> {
        let passages_lock = self.passages.lock().unwrap(); // Acquire the lock
        passages_lock.get(&id).cloned() // Perform the get operation
    }

    fn get_passages_for_location(&self, location_id: i32) -> Vec<Passage> {
        let passages_lock = self.passages.lock().unwrap(); // Acquire the lock
        passages_lock.values()
            .filter(|passage| passage.get_from_location() == location_id || passage.get_to_location() == location_id)
            .cloned()
            .collect() // Collect filtered and cloned passages into a Vec
    }
    fn find_passage_by_location_and_direction(&self, location_id: i32, direction: &str) -> Option<Passage> {
        let passages = self.passages.lock().unwrap();
        passages.values().find(|&passage|
            (passage.get_from_location() == location_id) && (passage.get_direction_reference().eq_ignore_ascii_case(direction))
        ).cloned()
    }

    fn add_passage(&self, passage: Passage) -> Result<(), String> {
        let mut locations = self.passages.lock().map_err(|_| "Mutex lock failed")?;
        locations.insert(passage.get_aggregate_id(), passage);
        Ok(())
    }

    fn find_by_start_and_end_id(&self, from_location_id: i32, to_location_id: i32) -> Option<Passage> {
        self.passages.lock().unwrap().values()
            .find(|passage| passage.get_from_location() == from_location_id && passage.get_to_location() == to_location_id)
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
    fn find_by_id(&self, id: i32) -> Option<PlayerState> {
        let states = self.states.lock().unwrap();
        states.get(&id).cloned()
    }

    fn save(&self, player_state: PlayerState) {
        let mut states = self.states.lock().unwrap();
        println!("Updating InMemoryRepositoryPlayerState with {:?}", &player_state);
        states.insert(player_state.get_player_id(), player_state);
    }
}