pub struct RequestContext {
    pub player_id: Option<u64>, // Adjust the type based on your identification strategy
    // Other fields as needed
}

impl RequestContext {
    pub fn new(player_id: Option<u64>) -> Self {
        RequestContext { player_id }
    }
}