use domain_pure::model::player_state::PlayerState;

pub trait PlayerStateRepository: Send + Sync {
    fn find_by_id(&self, id: i32) -> Option<PlayerState>;
    fn save(&self, player_state: PlayerState);
    // Define additional methods as needed
}