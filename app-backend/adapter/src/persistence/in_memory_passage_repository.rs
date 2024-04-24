use std::collections::HashMap;
use std::fmt;
use tokio::sync::Mutex;
use std::sync::Arc;
use futures::future::BoxFuture;
use port::dto::passage_dto::PassageDTO;
use port::repositories::passage_repository::PassageRepository;
use crosscutting::error_management::error::Error;
use crate::persistence::in_memory_location_repository::InMemoryLocationRepository;

#[derive(Clone)]
pub struct InMemoryPassageRepository {
    passages: Arc<Mutex<HashMap<i32, PassageDTO>>>,
}

impl InMemoryPassageRepository {
    pub fn new() -> Self {
        Self {
            passages: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl fmt::Debug for InMemoryPassageRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InMemoryPassageRepository")
            .finish()
    }
}

impl PassageRepository for InMemoryPassageRepository {
    fn get_passage_by_id(&self, id: i32) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>> {
        let passages = self.passages.clone();
        Box::pin(async move {
            let lock = passages.lock().await;
            Ok(lock.get(&id).cloned())
        })
    }

    fn get_passages_for_location(&self, location_id: i32) -> BoxFuture<'static, Result<Vec<PassageDTO>, Error>> {
        let passages = self.passages.clone();
        Box::pin(async move {
            let lock = passages.lock().await;
            Ok(lock.values()
                .filter(|passage| passage.from_location_id == location_id || passage.to_location_id == location_id)
                .cloned()
                .collect())
        })
    }

    fn find_passage_by_location_and_direction(&self, location_id: i32, direction: &str) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>> {
        let passages = self.passages.clone();
        let direction = direction.to_owned();  // Clone the direction into a new String

        Box::pin(async move {
            let lock = passages.lock().await;
            Ok(lock.values()
                .find(|&passage| (passage.from_location_id == location_id) && (passage.direction.eq_ignore_ascii_case(&direction)))
                .cloned())
        })
    }

    fn add_passage(&self, passage: PassageDTO) -> BoxFuture<'static, Result<(), Error>> {
        let passages = self.passages.clone();
        Box::pin(async move {
            let mut lock = passages.lock().await;
            lock.insert(passage.id, passage);
            Ok(())
        })
    }

    fn find_by_start_and_end_id(&self, from_location_id: i32, to_location_id: i32) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>> {
        let passages = self.passages.clone();
        Box::pin(async move {
            let lock = passages.lock().await;
            Ok(lock.values()
                .find(|passage| passage.from_location_id == from_location_id && passage.to_location_id == to_location_id)
                .cloned())
        })
    }
}
