use std::collections::HashMap;
use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::Mutex;

use crosscutting::error_management::error::Error;
use port::dto::location_dto::LocationDTO;
use port::repositories::location_repository::LocationRepository;

#[derive(Clone)]
pub struct InMemoryLocationRepository {
    locations: Arc<Mutex<HashMap<u64, HashMap<u64, LocationDTO>>>>,
}

impl InMemoryLocationRepository {
    pub fn new() -> Self {
        Self {
            locations: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl LocationRepository for InMemoryLocationRepository {
    fn get_location_by_id(&self, game_id: u64, location_id: u64) -> BoxFuture<'static, Result<Option<LocationDTO>, Error>> {
        let locations = self.locations.clone();

        Box::pin(async move {
            let locations = locations.lock().await;
            Ok(locations.get(&game_id).and_then(|game_locations| game_locations.get(&location_id).cloned()))
        })
    }

    fn get_all_locations(&self, game_id: u64) -> BoxFuture<'static, Result<Vec<LocationDTO>, Error>> {
        let locations = self.locations.clone();

        Box::pin(async move {
            let locations = locations.lock().await;
            match locations.get(&game_id) {
                Some(game_locations) => Ok(game_locations.values().cloned().collect()),
                None => Ok(vec![]), // Return an empty vector if no locations are found for the game
            }
        })
    }

    fn add_location(&self, game_id: u64, location: LocationDTO) -> BoxFuture<'static, Result<(), Error>> {
        let locations = self.locations.clone();

        Box::pin(async move {
            let mut locations = locations.lock().await;
            let game_locations = locations.entry(game_id).or_insert_with(HashMap::new);
            game_locations.insert(location.id, location);
            Ok(())
        })
    }
}

impl std::fmt::Debug for InMemoryLocationRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InMemoryLocationRepository").finish()
    }
}