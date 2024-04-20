use std::sync::Arc;
use crate::repositories::location_repository::LocationRepository;
use crate::repositories::passage_repository::PassageRepository;
use crate::repositories::player_state_repository::PlayerStateRepository;


#[derive(Clone)]
pub struct OutboundPorts {
    location_repo: Arc<dyn LocationRepository>,
    passage_repo: Arc<dyn PassageRepository>,
    player_state: Arc<dyn PlayerStateRepository>,
}

impl OutboundPorts {
    pub fn new(
        location_repo: Arc<dyn LocationRepository>,
        passage_repo: Arc<dyn PassageRepository>,
        player_state_repo: Arc<dyn PlayerStateRepository>,
    ) -> Self {

        let location_repo = location_repo;
        let passage_repo =passage_repo;
        let player_state_repo =player_state_repo;

        OutboundPorts {
            location_repo: location_repo,
            passage_repo: passage_repo,
            player_state: player_state_repo,
        }
    }

    pub fn location_repo(&self) -> Arc<dyn LocationRepository> {
        self.location_repo.clone()
    }

    pub fn passage_repo(&self) -> Arc<dyn PassageRepository> {
        self.passage_repo.clone()
    }

    pub fn player_state_repo(&self) -> Arc<dyn PlayerStateRepository> {
        self.player_state.clone()
    }
}
