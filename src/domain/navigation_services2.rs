// src/domain/navigation_services.rs

use crate::domain::aggregates::player_state::PlayerState;
use crate::port::repository::{LocationRepository, PassageRepository};
use std::sync::Arc;

pub struct NavigationService<R: LocationRepository, P: PassageRepository> {
    location_repository: Arc<R>,
    passage_repository: Arc<P>,
}

impl<R, P> NavigationService<R, P>
    where
        R: LocationRepository,
        P: PassageRepository,
{
    pub fn new() -> Self {
        NavigationService {
            location_repository,
            passage_repository,
        }
    }

    // Simplified movement logic
    pub fn update_location_based_on_direction(
        &self,
        player_state: &mut PlayerState,
        _direction: &str,
    ) -> Result<(), &'static str> {
        // For now, we're ignoring the direction and just setting the location to a fixed ID
        let fixed_target_location_id = 1;

        // Check if the target location exists in the repository
        if self.location_repository.get_location_by_id(fixed_target_location_id).is_some() {
            player_state.set_current_location_id(fixed_target_location_id);
            Ok(())
        } else {
            Err("The target location does not exist.")
        }
    }
}
