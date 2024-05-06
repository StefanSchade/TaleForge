use std::collections::HashMap;
use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::Mutex;

use crosscutting::error_management::error::Error;
use port::dto::bout_dto::BoutDTO;
use port::repositories::bout_repository::BoutRepository;

#[derive(Clone)]
pub struct InMemoryBoutRepository {
    bouts: Arc<Mutex<HashMap<i64, BoutDTO>>>,
}

impl InMemoryBoutRepository {
    pub fn new() -> Self {
        Self {
            bouts: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl BoutRepository for InMemoryBoutRepository {
    fn get_bout_by_id(&self, match_id: i64) -> BoxFuture<'static, Result<Option<BoutDTO>, Error>> {
        let bouts = self.bouts.clone();

        Box::pin(async move {
            let bouts = bouts.lock().await;
            Ok(bouts.get(&match_id).cloned()) // Return a clone of the BoutDTO if found
        })
    }

    fn add_bout(&self, bout: BoutDTO) -> BoxFuture<'static, Result<(), Error>> {
        let bouts = self.bouts.clone();

        Box::pin(async move {
            let mut lock = bouts.lock().await;
            lock.insert(bout.id, bout);
            Ok(())
        })
    }
}

impl std::fmt::Debug for InMemoryBoutRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InMemoryBoutRepository").finish()
    }
}