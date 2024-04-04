use domain_contract::contracts::passage_queries::navigation::PassageQueries;
use domain_pure::model::passage::Passage;


use port::repositories::passage_repository::PassageRepository;
use std::sync::Arc;

pub struct PassageQueryImpl {
    passage_repository: Arc<dyn PassageRepository>,
}

impl PassageQueryImpl {
    pub fn new(
        passage_repository: Arc<dyn PassageRepository>
    ) -> Self {
        PassageQueryImpl {
            passage_repository,
        }
    }
}

impl PassageQueries for PassageQueryImpl {
    fn find_passage_between_locations(&self, from_location_id: i32, to_location_id: i32) -> Option<Passage> {
        self.passage_repository.find_by_start_and_end_id(from_location_id, to_location_id)
    }


    fn find_passage_by_location_and_direction(&self, location_id: i32, direction: &str) -> Option<Passage> {
        self.passage_repository.find_passage_by_location_and_direction(location_id, direction)
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