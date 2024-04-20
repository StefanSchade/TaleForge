use std::sync::Arc;

use domain_contract::contracts::location_queries::navigation::LocationQueries;
use domain_pure::model::location::Location;
use port::repositories::location_repository::LocationRepository;

use crate::dto_domain_mapping::location_mapper::location_map_dto_to_domain;

pub struct LocationQueryImpl {
    //   location_repository: Arc<dyn LocationRepository>,
    location_repository: Arc<dyn LocationRepository>,
}

impl LocationQueryImpl {
    pub fn new(
location_repository: Arc<dyn LocationRepository>
    ) -> Self {
        LocationQueryImpl {
            location_repository,
        }
    }
}

impl LocationQueries for LocationQueryImpl {
    fn get_location_by_aggregate_id(&self, location_aggregate_id: i32) -> Option<Location> {
        let location_option = self.location_repository.get_location_by_id(location_aggregate_id);
        location_option.map(|location_dto| location_map_dto_to_domain(location_dto))
    }
}