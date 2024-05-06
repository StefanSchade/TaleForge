use std::sync::Arc;
use crate::repositories::bout_repository::BoutRepository;

use crate::repositories::location_repository::LocationRepository;
use crate::repositories::passage_repository::PassageRepository;
use crate::repositories::player_state_repository::PlayerStateRepository;

#[derive(Clone, Debug)]
pub struct OutboundAdapters {
    location_repo: Arc<dyn LocationRepository>,
    passage_repo: Arc<dyn PassageRepository>,
    player_state: Arc<dyn PlayerStateRepository>,
    bout_repo: Arc<dyn BoutRepository>
}

impl OutboundAdapters {
    pub fn new(
        location_repo: Arc<dyn LocationRepository>,
        passage_repo: Arc<dyn PassageRepository>,
        player_state_repo: Arc<dyn PlayerStateRepository>,
        bout_repo: Arc<dyn BoutRepository>,
    ) -> Self {
        let location_repo = location_repo;
        let passage_repo = passage_repo;
        let player_state_repo = player_state_repo;

        OutboundAdapters {
            location_repo: location_repo,
            passage_repo: passage_repo,
            player_state: player_state_repo,
            bout_repo: bout_repo
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

    pub fn bout_repo(&self) -> Arc<dyn BoutRepository> { self.bout_repo.clone()
    }
}