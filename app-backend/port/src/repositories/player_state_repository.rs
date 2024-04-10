use crate::dto::location_dto::LocationDTO;
use crate::dto::player_state_dto::PlayerStateDTO;
use crate::repositories::location_repository::LocationRepository;

#[cfg_attr(test, mockall::automock)]
pub trait PlayerStateRepository: Send + Sync {
    fn find_by_id(&self, id: i32) -> Option<PlayerStateDTO>;
    fn save(&self, player_state: PlayerStateDTO);
    // Define additional methods as needed
}
