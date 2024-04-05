use crate::dto::location_dto::LocationDTO;

pub trait LocationRepository: Send + Sync  {
    fn get_location_by_id(&self, id: i32) -> Option<LocationDTO>;
    fn get_all_locations(&self) -> Vec<LocationDTO>;
    fn add_location(&self, location: LocationDTO) -> Result<(), String>;
    // Add more methods as needed


}
