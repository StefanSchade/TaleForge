use domain_pure::model::location::Location;

pub trait LocationRepository: Send + Sync  {
    fn get_location_by_id(&self, id: i32) -> Option<Location>;
    fn get_all_locations(&self) -> Vec<Location>;
    fn add_location(&self, location: Location) -> Result<(), String>;
    // Add more methods as needed


}
