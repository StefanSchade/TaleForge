pub mod navigation {
    use crate::model::location::Location;

    pub trait LocationQueries: Send + Sync {
        fn get_location_by_aggregate_id(&self, location_aggregate_id: i32) -> Option<Location>;
        //fn find_location_by_direction(&self, current_location_id: i32, direction: &str) -> Option<Location>;
    }
}
