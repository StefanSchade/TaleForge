use std::sync::Arc;

use crate::domain::aggregates::location::Location;
use crate::domain::aggregates::player_state::PlayerState;
use crate::port::repository::{LocationRepository, PassageRepository};

pub struct NavigationService {
    location_repository: Arc<dyn LocationRepository>,
    passage_repository: Arc<dyn PassageRepository>,
}

impl NavigationService {
    pub fn new(location_repository: Arc<dyn LocationRepository>, passage_repository: Arc<dyn PassageRepository>) -> Self {
        NavigationService { location_repository, passage_repository }
    }

    // Example method to navigate based on direction
    pub fn navigate(&self, player_state: PlayerState, direction: String) -> Result<(Location, String), String> {

        if let Some(passage) = self.passage_repository.find_passage_by_direction_and_location(player_state.current_location_id, &*direction) {
            if let Some(target_location) = self.location_repository.get_location_by_id(passage.to_location_id) {
                let narration = format!("You {} and reach {}.", passage.narration, target_location.title);
                Ok((target_location, narration))
            } else {
                Err("Target location not found.".to_string())
            }
        } else {
            Err("No passage found in that direction.".to_string())
        }
    }
}
