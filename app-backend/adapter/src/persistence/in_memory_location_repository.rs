use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::future::BoxFuture;
use crosscutting::error_management::error::Error;
use port::dto::location_dto::LocationDTO;
use port::repositories::location_repository::LocationRepository;

#[derive(Clone)]
pub struct InMemoryLocationRepository {
    locations: Arc<Mutex<HashMap<i32, LocationDTO>>>,
}

impl InMemoryLocationRepository {
    pub fn new() -> Self {
        Self {
            locations: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl fmt::Debug for InMemoryLocationRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InMemoryLocationRepository")
            .finish()
    }
}

impl LocationRepository for InMemoryLocationRepository {
    fn get_location_by_id(&self, id: i32) -> BoxFuture<'static, Result<Option<LocationDTO>, Error>> {
        let locations = Arc::clone(&self.locations);

        Box::pin(async move {
            let guard = locations.lock().await; // directly gets a MutexGuard
            Ok(guard.get(&id).cloned())  // No need for match or Result handling here
        })
    }

    fn get_all_locations(&self) -> BoxFuture<'static, Result<Vec<LocationDTO>, Error>> {
        let locations = Arc::clone(&self.locations);
        Box::pin(async move {
            let guard = locations.lock().await; // directly gets a MutexGuard
            Ok(guard.values().cloned().collect())  // Collect all locations
        })
    }

    fn add_location(&self, location: LocationDTO) -> BoxFuture<'static, Result<(), Error>> {
        let locations = Arc::clone(&self.locations);
        Box::pin(async move {
            let mut guard = locations.lock().await; // directly gets a MutexGuard
            guard.insert(location.id, location);
            Ok(())
        })
    }
}
