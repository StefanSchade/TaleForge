use std::{fmt, future};
use std::fmt::Debug;

use futures::future::BoxFuture;
use futures::FutureExt;

use crosscutting::error_management::error::Error;

use crate::dto::location_dto::LocationDTO;

pub trait LocationRepository: Send + Sync + Debug {
    fn get_location_by_id(&self, id: i32) -> BoxFuture<'static, Result<Option<LocationDTO>, Error>>;
    fn get_all_locations(&self) -> BoxFuture<'static, Result<Vec<LocationDTO>, Error>>;
    fn add_location(&self, location: LocationDTO) -> BoxFuture<'static, Result<(), Error>>;
}

#[cfg(feature = "test-utils")]
pub struct MockLocationRepository {
    pub fixed_location: LocationDTO,
    pub all_locations: Option<Vec<LocationDTO>>,
}

#[cfg(feature = "test-utils")]
impl MockLocationRepository {
    pub fn new(fixed_location: LocationDTO, all_locations: Option<Vec<LocationDTO>>) -> Self {
        MockLocationRepository {
            fixed_location,
            all_locations,
        }
    }
}

#[cfg(feature = "test-utils")]
impl LocationRepository for MockLocationRepository {
    #[cfg(feature = "test-utils")]
    fn get_location_by_id(&self, id: i32) -> BoxFuture<'static, Result<Option<LocationDTO>, Error>> {
        let fixed_location = self.fixed_location.clone();
        future::ready(
            Ok(
            if id == fixed_location.id {
                Some(fixed_location)
            } else {
                None
            }
        )).boxed()
    }

    fn get_all_locations(&self) ->  BoxFuture<'static, Result<Vec<LocationDTO>, Error>> {
        future::ready(Ok(self.all_locations.clone().unwrap())).boxed()
    }

    fn add_location(&self, _location: LocationDTO) -> BoxFuture<'static, Result<(), Error>> {
        future::ready(Ok(())).boxed()
    }
}

#[cfg(feature = "test-utils")]
impl fmt::Debug for MockLocationRepository {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MockLocationRepository")
            .finish()  // Adjust according to what you might want to show in debug
    }
}

#[tokio::test]
async fn test_with_mock_repository() {
    let fixed_location = LocationDTO {
        id: 1,
        title: "title1".to_string(),
        description: "description1".to_string(),
        image_url: None,
    };

    let mock_repo = MockLocationRepository::new(fixed_location.clone(), None);
    let future = mock_repo.get_location_by_id(1); // Get the future
    let location = future.await.expect("Failed to get location"); // Await the future

    assert_eq!(location.unwrap().description, fixed_location.description);
}

#[tokio::test]
async fn test_get_all_locations() {
    let fixed_location = LocationDTO {
        id: 1,
        title: "title1".to_string(),
        description: "description1".to_string(),
        image_url: None,
    };

    let all_locations = vec![
        fixed_location.clone(),
        LocationDTO {
            id: 2,
            title: "title2".to_string(),
            description: "description2".to_string(),
            image_url: None,
        },
    ];

    let mock_repo = MockLocationRepository::new(fixed_location, Some(all_locations.clone()));
    let locations_future = mock_repo.get_all_locations();
    let locations = locations_future.await.expect("Failed to get locations");

    // Serialize both vectors to JSON strings for comparison
    let expected_json = serde_json::to_string(&all_locations).expect("Failed to serialize expected locations");
    let actual_json = serde_json::to_string(&locations).expect("Failed to serialize actual locations");

    assert_eq!(expected_json, actual_json);
}
