pub mod navigation {
    use crate::aggregates::passage::Passage;
    use crate::aggregates::location::Location;

    pub trait NavigationQuery  : Send + Sync  {
        fn find_passage_between_locations(&self, from_location_id: i32, to_location_id: i32) -> Option<Passage>;
        //fn find_location_by_direction(&self, current_location_id: i32, direction: &str) -> Option<Location>;
    }
}
