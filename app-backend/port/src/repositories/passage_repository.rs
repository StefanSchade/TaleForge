use domain::model::passage::Passage;

pub trait PassageRepository: Send + Sync {
    fn get_passage_by_id(&self, id: i32) -> Option<Passage>;
    fn get_passages_for_location(&self, location_id: i32) -> Vec<Passage>;
    // New method to find a passage by direction and current location
    fn find_passage_by_location_and_direction(&self, location_id: i32, direction: &str) -> Option<Passage>;
    fn add_passage(&self, passage: Passage) -> Result<(), String>;
    fn find_by_start_and_end_id(&self, from_location_id: i32, to_location_id:i32) -> Option<Passage>;
}
