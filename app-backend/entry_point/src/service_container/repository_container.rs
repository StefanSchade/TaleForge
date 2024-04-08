use std::sync::Arc;
use adapter::persistence::in_memory_repository::{InMemoryLocationRepository, InMemoryPassageRepository, InMemoryPlayerStateRepository};
use port::repositories::location_repository::LocationRepository;
use port::repositories::passage_repository::PassageRepository;
use port::repositories::player_state_repository::PlayerStateRepository;


#[derive(Clone)]
pub struct RepositoryContainer {
    location: Arc<dyn LocationRepository>,
    passage: Arc<dyn PassageRepository>,
    player_state: Arc<dyn PlayerStateRepository>,
}

impl RepositoryContainer {
    pub fn new(
//        location_repo: Arc<dyn LocationRepository>,
//        passage_repo: Arc<dyn PassageRepository>,
//        player_state_repo: Arc<dyn PlayerStateRepository>,
    ) -> Self {

        let location_repo = Arc::new(InMemoryLocationRepository::new());
        let passage_repo =Arc::new(InMemoryPassageRepository::new());
        let player_state_repo =Arc::new(InMemoryPlayerStateRepository::new());

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
