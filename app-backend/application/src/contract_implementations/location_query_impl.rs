

use domain_pure::model::location::Location;

use domain_contract::contracts::location_queries::navigation::LocationQueries;

use port::repositories::location_repository::LocationRepository;
use std::sync::Arc;


pub struct LocationQueryImpl {
 //   location_repository: Arc<dyn LocationRepository>,
    location_repository: Arc<dyn LocationRepository>,
}

impl LocationQueryImpl {
    pub fn new(
//        location_repository: Arc<dyn LocationRepository>,
        location_repository: Arc<dyn LocationRepository>
    ) -> Self {
        LocationQueryImpl {
             location_repository,
        }
    }
}

impl LocationQueries for LocationQueryImpl {
    fn get_location_by_aggregate_id(&self, location_aggregate_id: i32) -> Option<Location> {
        self.location_repository.get_location_by_id(location_aggregate_id)
    }

    // fn find_location_by_direction(&self, current_location_id: i32, direction: &str) -> Option<Location> {
    //     // This might involve more complex logic, potentially needing adjustments
    //     // in the repository interfaces to support direction-based contracts.
    //     // For simplicity, let's assume there's a method in the passage repository
    //     // to find passages by location and direction, then use the location repository
    //     // to fetch the target location.
    //     if let Some(passage) = self.passage_repository.find_by_location_and_direction(current_location_id, direction) {
    //       //  self.location_repository.find_by_id(passage.to_location_id)
    //         self.location_repository.find_by_id(passage.to_location_id)
    //     } else {
    //         None
    //     }
    // }
}