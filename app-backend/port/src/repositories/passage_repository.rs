#[cfg(feature = "test-utils")]
use std::{fmt, future};
use std::fmt::Debug;

use futures::future::BoxFuture;
#[cfg(feature = "test-utils")]
use futures::FutureExt;

use crosscutting::error_management::error::Error;

use crate::dto::passage_dto::PassageDTO;

pub trait PassageRepository: Send + Sync + Debug {
    fn get_passage_by_id(&self,  game_id: i64,  id: i64) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>>;
    fn get_passages_for_location(&self,  game_id: i64,   location_id: i64) -> BoxFuture<'static, Result<Vec<PassageDTO>, Error>>;
    // New method to find a passage by direction and current passage
    fn find_passage_by_location_and_direction(&self, game_id: i64,  location_id: i64, direction: &str) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>>;
    fn add_passage(&self, game_id: i64,  passage: PassageDTO) -> BoxFuture<'static, Result<(), Error>>;
    fn find_by_start_and_end_id(&self, game_id: i64,  from_location_id: i64, to_location_id: i64) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>>;
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
impl fmt::Debug for MockPassageRepository {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MockPassageRepository")
            .finish()  // Adjust according to what you might want to show in debug
    }
}

#[cfg(feature = "test-utils")]
impl PassageRepository for MockPassageRepository {
    #[cfg(feature = "test-utils")]
    fn get_passage_by_id(&self, game_id: i64,  id: i64) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>> {
        let fixed_passage = self.fixed_passage.clone();
        future::ready(
            Ok(
                if id == self.fixed_passage.id {
                    Some(fixed_passage)
                } else {
                    None
                }
            )
        ).boxed()
    }

    fn get_passages_for_location(&self,game_id: i64,  _from_location_id: i64) -> BoxFuture<'static, Result<Vec<PassageDTO>, Error>> {
        future::ready(
            Ok(
                self.all_passages_for_one_location.clone().unwrap()
            )
        ).boxed()
    }

    fn add_passage(&self, game_id: i64,  _passage: PassageDTO) -> BoxFuture<'static, Result<(), Error>> {
        future::ready(Ok(())).boxed()
    }

    fn find_passage_by_location_and_direction(&self, game_id: i64,  from_location_id: i64, _direction: &str) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>> {
        future::ready(
            Ok(
                if from_location_id == self.fixed_passage.from_location_id {
                    Some(self.fixed_passage.clone())
                } else {
                    None
                }
            )
        ).boxed()
    }

    fn find_by_start_and_end_id(&self, game_id: i64,  from_location_id: i64, to_location_id: i64) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>> {
        future::ready(
            Ok(
                if from_location_id == self.fixed_passage.from_location_id && to_location_id == self.fixed_passage.to_location_id {
                    Some(self.fixed_passage.clone())
                } else {
                    None
                }
            )
        ).boxed()
    }

}

#[tokio::test]
async fn test_with_mock_repository() {

    let expected_passage_id: i64 = 0;
    let expected_game: i64 = 3;
    let expected_original_location: i64 = 1;
    let expected_destination_location: i64 = 2;

    let fixed_passage = PassageDTO {
        id: expected_passage_id,
        game_id: expected_game,
        from_location_id: expected_original_location,
        to_location_id: expected_destination_location,
        description: "description1".to_string(),
        direction: "direction1".to_string(),
        narration: "narration1".to_string(),
    };

    let mock_repo = MockPassageRepository::new(fixed_passage, None);
    let passage_future = mock_repo.get_passage_by_id(expected_game, expected_passage_id);
    let passage = passage_future.await.expect("Failed to get passage");

    assert_eq!(passage.unwrap().description, "description1");
}

#[tokio::test]
async fn test_get_passages_for_location() {

    let expected_passage_id: i64 = 0;
    let expected_game: i64 = 3;
    let expected_original_location: i64 = 1;
    let expected_destination_location: i64 = 2;

    let fixed_passage = PassageDTO {
        id: expected_passage_id,
        game_id: expected_game,
        from_location_id: expected_original_location,
        to_location_id: expected_destination_location,
        description: "description1".to_string(),
        direction: "direction1".to_string(),
        narration: "narration1".to_string(),
    };

    let all_passages = vec![

        fixed_passage.clone(),

        PassageDTO {
            id: expected_passage_id+1,
            game_id: expected_game,
            from_location_id: expected_original_location,
            to_location_id: expected_destination_location+1,
            description: "description2".to_string(),
            direction: "direction2".to_string(),
            narration: "narration2".to_string(),
        },
    ];

    let mock_repo = MockPassageRepository::new(fixed_passage, Some(all_passages.clone()));
    let passages_future = mock_repo.get_passages_for_location(expected_game, expected_original_location);
    let passages = passages_future.await.expect("Failed to get passages");

    let expected_json = serde_json::to_string(&all_passages).expect("Failed to serialize expected passages");
    let actual_json = serde_json::to_string(&passages).expect("Failed to serialize actual passages");

    assert_eq!(expected_json, actual_json);
}