pub struct RequestContext {
    pub player_id: u64,
    pub bout_id: u64,
}

impl RequestContext {
    pub fn new(bout_id: u64, player_id: u64) -> Self {
        RequestContext {bout_id, player_id }
    }
}