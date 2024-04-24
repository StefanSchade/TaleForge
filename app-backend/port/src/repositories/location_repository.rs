use std::fmt::Debug;
use futures::future::BoxFuture;
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
    fn get_location_by_id(&self, id: i32) -> Option<LocationDTO> {
        if id == self.fixed_location.id {
            Some(self.fixed_location.clone())
        } else {
            None
        }
    }

    fn get_all_locations(&self) -> Vec<LocationDTO> {
        self.all_locations.clone().unwrap()
    }

    fn add_location(&self, _location: LocationDTO) -> Result<(), String> {
        Ok(())
    }
}

#[test]
fn test_with_mock_repository() {
    let fixed_location = LocationDTO {
        id: 1,
        title: "title1".to_string(),
        description: "description1".to_string(),
        image_url: None,
    };

    let mock_repo = MockLocationRepository::new(fixed_location, None);
    let location = mock_repo.get_location_by_id(1).unwrap();

    assert_eq!(location.description, "description1");
}

#[test]
fn test_get_all_locations() {
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

    let locations = mock_repo.get_all_locations();

    // Serialize both vectors to JSON strings for comparison
    let expected_json = serde_json::to_string(&all_locations).expect("Failed to serialize expected locations");
    let actual_json = serde_json::to_string(&locations).expect("Failed to serialize actual locations");

    assert_eq!(expected_json, actual_json);
}