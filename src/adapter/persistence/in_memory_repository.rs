use std::collections::HashMap;
use std::option::Option;
use std::sync::Mutex;

use crate::domain::aggregates::location::Location;
use crate::domain::aggregates::passage::Passage;
use crate::domain::aggregates::player_state::PlayerState;
use crate::port::repository::{LocationRepository, PassageRepository, PlayerStateRepository};

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
    fn get_passage_by_id(&self, id: i32) -> Option<Passage> {
        let passages_lock = self.passages.lock().unwrap(); // Acquire the lock
        passages_lock.get(&id).cloned() // Perform the get operation
    }

    fn get_passages_for_location(&self, location_id: i32) -> Vec<Passage> {
        let passages_lock = self.passages.lock().unwrap(); // Acquire the lock
        passages_lock.values()
            .filter(|passage| passage.from_location_id == location_id || passage.to_location_id == location_id)
            .cloned()
            .collect() // Collect filtered and cloned passages into a Vec
    }
    fn find_passage_by_direction_and_location(&self, location_id: i32, direction: &str) -> Option<Passage> {
        let passages = self.passages.lock().unwrap();
        passages.values().find(|&passage|
            (passage.from_location_id == location_id) && (passage.direction.eq_ignore_ascii_case(direction))
        ).cloned()
    }

    fn add_passage(&self, passage: Passage) -> Result<(), String> {
        let mut locations = self.passages.lock().map_err(|_| "Mutex lock failed")?;
        locations.insert(passage.id, passage);
        Ok(())
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
        println!("{:?}", &player_state);
        states.insert(player_state.player_id.clone(), player_state);
    }
}