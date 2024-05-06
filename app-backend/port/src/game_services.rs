use crate::port_services::domain_story_move_player::{MovePlayerDomainStoryRequest, MovePlayerDomainStoryResponse};

pub trait GameService {
    fn move_player(&self, input: MovePlayerDomainStoryRequest) -> Result<MovePlayerDomainStoryResponse, &'static str>;
}