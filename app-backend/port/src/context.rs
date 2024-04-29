pub struct RequestContext {
    pub player_id: u64,
    pub game_id: u64,
}

impl RequestContext {
    pub fn new(game_id: u64, player_id: u64) -> Self {
        RequestContext { game_id, player_id }
    }
}