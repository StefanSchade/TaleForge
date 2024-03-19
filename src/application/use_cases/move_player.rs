use std::sync::Arc;

use crate::domain::navigation_services::NavigationService;
use crate::port::context::RequestContext;
use crate::port::dto::{MovePlayerCommand, MovePlayerResult};
use crate::port::repository::{LocationRepository, PassageRepository, PlayerStateRepository};

pub struct MovePlayerUseCase {
    location_repository: Arc<dyn LocationRepository>,
    passage_repository: Arc<dyn PassageRepository>,
    player_state_repository: Arc<dyn PlayerStateRepository>,
    navigation_service: NavigationService,
}

impl MovePlayerUseCase {
    pub fn new(
        location_repository: Arc<dyn LocationRepository>,
        passage_repository: Arc<dyn PassageRepository>,
        player_state_repository: Arc<dyn PlayerStateRepository>,
        navigation_service: NavigationService,
    ) -> Self {
        Self { location_repository, passage_repository, player_state_repository, navigation_service }
    }

    pub fn execute(&self, input: MovePlayerCommand, context: RequestContext) -> Result<MovePlayerResult, String> {

        // For simplification, assume a fixed current location (e.g., location ID 1)
        let current_location_id = 1;

        // Use the NavigationService to determine the new location based on the direction
        match self.navigation_service.navigate(current_location_id, input.direction) {
            Ok((new_location, narration)) => {
                // Here, new_location would be an instance of the Location entity
                // which contains all the necessary details like image_url, description, etc.
                Ok(MovePlayerResult {
                    player_location: 1,
                    narration: "Something is happening".to_string(),
                })
            }
            Err(error) => Err(error.to_string()), // Convert the error to a String
        }
    }
}