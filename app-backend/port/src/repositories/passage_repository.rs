use futures::future::BoxFuture;
use crosscutting::error_management::error::Error;
use crate::dto::passage_dto::PassageDTO;

pub trait PassageRepository: Send + Sync {
    fn get_passage_by_id(&self, id: i32) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>>;
    fn get_passages_for_location(&self, location_id: i32) -> BoxFuture<'static, Result<Vec<PassageDTO>, Error>>;
    // New method to find a passage by direction and current passage
    fn find_passage_by_location_and_direction(&self, location_id: i32, direction: &str) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>>;
    fn add_passage(&self, passage: PassageDTO) -> BoxFuture<'static, Result<(), Error>>;
    fn find_by_start_and_end_id(&self, from_location_id: i32, to_location_id: i32) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>>;
}

#[cfg(feature = "test-utils")]
pub struct MockPassageRepository {
    pub fixed_passage: PassageDTO,
    pub all_passages_for_one_location: Option<Vec<PassageDTO>>,
}

#[cfg(feature = "test-utils")]
impl MockPassageRepository {
    pub fn new(fixed_passage: PassageDTO, all_passages_for_one_location: Option<Vec<PassageDTO>>) -> Self {
        MockPassageRepository {
            fixed_passage,
            all_passages_for_one_location,
        }
    }
}

#[cfg(feature = "test-utils")]
impl PassageRepository for MockPassageRepository {
    #[cfg(feature = "test-utils")]
    fn get_passage_by_id(&self, id: i32) -> Option<PassageDTO> {
        if id == self.fixed_passage.id {
            Some(self.fixed_passage.clone())
        } else {
            None
        }
    }

    fn get_passages_for_location(&self, _from_location_id: i32) -> Vec<PassageDTO> {
        self.all_passages_for_one_location.clone().unwrap()
    }

    fn add_passage(&self, _passage: PassageDTO) -> Result<(), String> {
        Ok(())
    }

    fn find_passage_by_location_and_direction(&self, from_location_id: i32, _direction: &str) -> Option<PassageDTO> {
        if from_location_id == self.fixed_passage.from_location_id {
            Some(self.fixed_passage.clone())
        } else {
            None
        }
    }

    fn find_by_start_and_end_id(&self, from_location_id: i32, to_location_id: i32) -> Option<PassageDTO> {
        if from_location_id == self.fixed_passage.from_location_id && to_location_id == self.fixed_passage.to_location_id {
            Some(self.fixed_passage.clone())
        } else {
            None
        }
    }
}

#[test]
fn test_with_mock_repository() {
    let fixed_passage = PassageDTO {
        id: 1,
        from_location_id: 1,
        to_location_id: 2,
        description: "description1".to_string(),
        direction: "direction1".to_string(),
        narration: "narration1".to_string(),
    };

    let mock_repo = MockPassageRepository::new(fixed_passage, None);
    let passage = mock_repo.get_passage_by_id(1).unwrap();

    assert_eq!(passage.description, "description1");
}

#[test]
fn test_get_passages_for_location() {
    let fixed_passage = PassageDTO {
        id: 1,
        from_location_id: 1,
        to_location_id: 2,
        description: "description1".to_string(),
        direction: "direction1".to_string(),
        narration: "narration1".to_string(),
    };

    let all_passages = vec![
        fixed_passage.clone(),
        PassageDTO {
            id: 2,
            from_location_id: 1,
            to_location_id: 3,
            description: "description2".to_string(),
            direction: "direction2".to_string(),
            narration: "narration2".to_string(),
        },
    ];

    let mock_repo = MockPassageRepository::new(fixed_passage, Some(all_passages.clone()));
    let passages = mock_repo.get_passages_for_location(1);

    let expected_json = serde_json::to_string(&all_passages).expect("Failed to serialize expected passages");
    let actual_json = serde_json::to_string(&passages).expect("Failed to serialize actual passages");

    assert_eq!(expected_json, actual_json);
}