use crate::port::repository::{LocationRepository, PassageRepository};
use crate::domain::aggregates::location::Location;
use crate::domain::aggregates::passage::Passage;
use std::collections::HashMap;
use std::option::Option;

pub struct InMemoryLocationRepository {
    locations: HashMap<i32, Location>,
}

impl InMemoryLocationRepository {
    pub fn new() -> Self {
        // Initialize with test data or leave empty to be filled dynamically
        Self {
            locations: HashMap::new(),
        }
    }
}

impl LocationRepository for InMemoryLocationRepository {
    fn get_location_by_id(&self, id: i32) -> Option<Location> {
        self.locations.get(&id).cloned()
    }

    fn get_all_locations(&self) -> Vec<Location> {
        self.locations.values().cloned().collect()
    }
}


pub struct InMemoryPassageRepository {
    passages: HashMap<i32, Passage>
}


impl InMemoryPassageRepository {
    pub fn new() -> Self {
        // Initialize with test data or leave empty to be filled dynamically
        Self {
            passages: HashMap::new(),
        }
    }
}


impl PassageRepository for InMemoryPassageRepository {
    fn get_passage_by_id(&self, id: i32) -> Option<Passage> {
        self.passages.get(&id).cloned()
    }

    fn get_passages_for_location(&self, location_id: i32) -> Vec<Passage> {
        self.passages.values().cloned().collect()
    }
}