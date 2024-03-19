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

        if let Some(player_id) = context.player_id {
            let player_state = match self.player_state_repository.find_by_id(player_id) {
                Some(state) => state,
                None => return Err("Player state not found".to_string()),
            };

            let new_location = self.navigation_service.navigate(player_state, input.direction)?;

            // Optionally, update the player's state with the new location
            // self.player_state_repository.save(updated_player_state)?;

            Ok(MovePlayerResult {
                player_location: 1,
                narration: "Something is happening".to_string(),
            })
        } else {
            Err("Player ID not found in context".to_string())
        }
    }

}