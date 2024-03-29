use domain::model::location::Location;
use domain::model::passage::Passage;
use domain::model::player_state::PlayerState;

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
    fn find_passage_by_location_and_direction(&self, location_id: i32, direction: &str) -> Option<Passage>;
    fn add_passage(&self, passage: Passage) -> Result<(), String>;
    fn find_by_start_and_end_id(&self, from_location_id: i32, to_location_id:i32) -> Option<Passage>;
}

pub trait PlayerStateRepository: Send + Sync {
    fn find_by_id(&self, id: i32) -> Option<PlayerState>;
    fn save(&self, player_state: PlayerState);
    // Define additional methods as needed
}