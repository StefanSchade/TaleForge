use std::sync::Arc;
use crate::repositories::location_repository::LocationRepository;
use crate::repositories::passage_repository::PassageRepository;
use crate::repositories::player_state_repository::PlayerStateRepository;


#[derive(Clone)]
pub struct RepositoryContainer {
    location: Arc<dyn LocationRepository>,
    passage: Arc<dyn PassageRepository>,
    player_state: Arc<dyn PlayerStateRepository>,
}

impl RepositoryContainer {
    pub fn new(
        location_repo: Arc<dyn LocationRepository>,
        passage_repo: Arc<dyn PassageRepository>,
        player_state_repo: Arc<dyn PlayerStateRepository>,
    ) -> Self {

        let location_repo = location_repo;
        let passage_repo =passage_repo;
        let player_state_repo =player_state_repo;

        RepositoryContainer {
            location: location_repo,
            passage: passage_repo,
            player_state: player_state_repo,
        }
    }

    pub fn location(&self) -> Arc<dyn LocationRepository> {
        self.location.clone()
    }

    pub fn passage(&self) -> Arc<dyn PassageRepository> {
        self.passage.clone()
    }

    pub fn player_state(&self) -> Arc<dyn PlayerStateRepository> {
        self.player_state.clone()
    }
}
