use crate::port::repository::{LocationRepository, PassageRepository};
use std::sync::Arc;

pub struct AppState {
    pub location_repository: Arc<dyn LocationRepository>,
    pub passage_repository: Arc<dyn PassageRepository>,
}

impl AppState {
    pub fn new(location_repository: Arc<dyn LocationRepository>, passage_repository: Arc<dyn PassageRepository>) -> Self {
        AppState { location_repository, passage_repository }
    }
}