use std::collections::HashMap;
use std::sync::Arc;
use std::fmt;
use futures::future::BoxFuture;
use tokio::sync::Mutex;

use crosscutting::error_management::error::Error;
use port::dto::passage_dto::PassageDTO;
use port::repositories::passage_repository::PassageRepository;

#[derive(Clone)]
pub struct InMemoryPassageRepository {
    passages: Arc<Mutex<HashMap<u64, HashMap<u64, PassageDTO>>>>,
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
        f.debug_struct("InMemoryPassageRepository").finish()
    }
}

impl PassageRepository for InMemoryPassageRepository {
    fn get_passage_by_id(&self, game_id: u64, id: u64) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>> {
        let passages = self.passages.clone();
        Box::pin(async move {
            let passages = passages.lock().await;
            Ok(passages.get(&game_id).and_then(|game_passages| game_passages.get(&id).cloned()))
        })
    }

    fn get_passages_for_location(&self, game_id: u64, location_id: u64) -> BoxFuture<'static, Result<Vec<PassageDTO>, Error>> {
        let passages = self.passages.clone();
        Box::pin(async move {
            let passages = passages.lock().await;
            Ok(passages.get(&game_id).map_or(vec![], |game_passages| {
                game_passages.values()
                    .filter(|p| p.from_location_id == location_id || p.to_location_id == location_id)
                    .cloned()
                    .collect()
            }))
        })
    }

    fn find_passage_by_location_and_direction(&self, game_id: u64, location_id: u64, direction: &str) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>> {
        let passages = self.passages.clone();
        let direction = direction.to_owned();
        Box::pin(async move {
            let passages = passages.lock().await;
            Ok(passages.get(&game_id).and_then(|game_passages| {
                game_passages.values()
                    .find(|p| p.from_location_id == location_id && p.direction.eq_ignore_ascii_case(&direction))
                    .cloned()
            }))
        })
    }

    fn add_passage(&self, game_id: u64, passage: PassageDTO) -> BoxFuture<'static, Result<(), Error>> {
        let passages = self.passages.clone();
        Box::pin(async move {
            let mut passages = passages.lock().await;
            let game_passages = passages.entry(game_id).or_insert_with(HashMap::new);
            game_passages.insert(passage.id, passage);
            Ok(())
        })
    }

    fn find_by_start_and_end_id(&self, game_id: u64, from_location_id: u64, to_location_id: u64) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>> {
        let passages = self.passages.clone();
        Box::pin(async move {
            let passages = passages.lock().await;
            Ok(passages.get(&game_id).and_then(|game_passages| {
                game_passages.values()
                    .find(|p| p.from_location_id == from_location_id && p.to_location_id == to_location_id)
                    .cloned()
            }))
        })
    }
}