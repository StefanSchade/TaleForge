use crate::domain::aggregates::location::Location;
use crate::domain::aggregates::passage::Passage;

// we want to be able to change the implementation of theese repos after compilation, therefore the traits need to be object safe. This implies that they have no method, that returns 'Self'. Compare implmementation of struc AppState

pub trait LocationRepository: Send + Sync  {
    fn get_location_by_id(&self, id: i32) -> Option<Location>;
    fn get_all_locations(&self) -> Vec<Location>;
    fn add_location(&self, location: Location) -> Result<(), String>;
    // Add more methods as needed
}


pub trait PassageRepository: Send + Sync {
    fn get_passage_by_id(&self, id: i32) -> Option<Passage>;
    fn get_passages_for_location(&self, location_id: i32) -> Vec<Passage>;
    // New method to find a passage by direction and current location
    fn find_passage_by_direction_and_location(&self, location_id: i32, direction: &str) -> Option<Passage>;
    fn add_passage(&self, passage: Passage) -> Result<(), String>;
}