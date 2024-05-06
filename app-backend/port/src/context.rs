pub struct RequestContext {
    pub player_id: i64,
    pub bout_id: i64,
}

impl RequestContext {
    pub fn new(bout_id: i64, player_id: i64) -> Self {
        RequestContext {bout_id, player_id }
    }
}