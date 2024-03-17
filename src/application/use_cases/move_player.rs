use std::sync::Arc;
use crate::domain::navigation_services::NavigationService;
use crate::port::dto::MovePlayerOutput;
use crate::port::repository::{LocationRepository, PassageRepository};

pub struct MovePlayerUseCase {
    location_repository: Arc<dyn LocationRepository>,
    passage_repository: Arc<dyn PassageRepository>,
    navigation_service: NavigationService,
}

impl MovePlayerUseCase {
    // Adjust the constructor to accept the NavigationService
    pub fn new(
        location_repository: Arc<dyn LocationRepository>,
        passage_repository: Arc<dyn PassageRepository>,
        navigation_service: NavigationService,
    ) -> Self {
        Self { location_repository, passage_repository, navigation_service }
    }

    pub fn execute(&self, direction: String) -> Result<MovePlayerOutput, String> {
        // For simplification, assume a fixed current location (e.g., location ID 1)
        let current_location_id = 1;

        // Use the NavigationService to determine the new location based on the direction
        match self.navigation_service.navigate(current_location_id, &direction) {
            Ok(new_location) => {
                // Here, new_location would be an instance of the Location entity
                // which contains all the necessary details like image_url, description, etc.
                Ok(MovePlayerOutput {
                    room_number: new_location.id,
                    title: new_location.title,
                    description: new_location.description,
                    image_url: new_location.image_url,
                })
            },
            Err(error) => Err(error),
        }
    }
}
