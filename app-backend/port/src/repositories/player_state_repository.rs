use crate::dto::player_state_dto::PlayerStateDTO;

pub trait PlayerStateRepository: Send + Sync {
    fn find_by_id(&self, id: i32) -> Option<PlayerStateDTO>;
    fn save(&self, player_state: PlayerStateDTO);
    // Define additional methods as needed
}