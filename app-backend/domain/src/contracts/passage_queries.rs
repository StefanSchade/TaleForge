pub mod navigation {
    use crate::model::passage::Passage;

    pub trait PassageQueries: Send + Sync {
        fn find_passage_between_locations(&self, from_location_id: i32, to_location_id: i32) -> Option<Passage>;

        fn find_passage_by_location_and_direction(&self, location_id: i32, direction: &str) -> Option<Passage>;


        //fn find_location_by_direction(&self, current_location_id: i32, direction: &str) -> Option<Location>;
    }
}
