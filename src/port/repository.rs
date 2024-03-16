use crate::domain::aggregates::location::Location;
use crate::domain::aggregates::passage::Passage;
pub trait LocationRepository {
    fn get_location_by_id(&self, id: i32) -> Option<Location>;
    fn get_all_locations(&self) -> Vec<Location>;
    // Add more methods as needed
}

pub trait PassageRepository {
    fn get_passage_by_id(&self, id: i32) -> Option<Passage>;
    fn get_passages_for_location(&self, location_id: i32) -> Vec<Passage>;
    // Add more methods as needed
}