use futures::future::BoxFuture;
use crosscutting::error_management::error::Error;
use crate::dto::player_state_dto::PlayerStateDTO;

pub trait PlayerStateRepository: Send + Sync {
    fn find_by_player_id(&self, id: i32) -> BoxFuture<'static, Result<Option<PlayerStateDTO>, Error>>;
    fn save(&self, player_state: PlayerStateDTO) -> BoxFuture<'static, Result<(), Error>>;
}
#[cfg(feature = "test-utils")]
pub struct MockPlayerStateRepository {
    pub fixed_player_state: PlayerStateDTO,
}

#[cfg(feature = "test-utils")]
impl MockPlayerStateRepository {
    pub fn new(fixed_player_state: PlayerStateDTO) -> Self {
        MockPlayerStateRepository {
            fixed_player_state,
        }
    }
}

#[cfg(feature = "test-utils")]
impl PlayerStateRepository for MockPlayerStateRepository {
    #[cfg(feature = "test-utils")]
    fn find_by_player_id(&self, player_id: i32) -> Option<PlayerStateDTO> {
        if player_id == self.fixed_player_state.player_id {
            Some(self.fixed_player_state.clone())
        } else {
            None
        }
    }

    fn save(&self, _player_state: PlayerStateDTO) {  }
}

#[test]
fn test_with_mock_repository() {
    let fixed_player_state = PlayerStateDTO {
        player_id: 1,
        current_location_id: 1,
    };

    let mock_repo = MockPlayerStateRepository::new(fixed_player_state);
    let player_state = mock_repo.find_by_player_id(1).unwrap();

    assert_eq!(player_state.current_location_id,1);
}