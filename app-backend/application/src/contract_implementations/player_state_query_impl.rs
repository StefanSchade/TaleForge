use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use crosscutting::error_management::error::Error;
use domain_contract::contracts::player_state_query::PlayerStateQuery;
use domain_pure::model::player_state::PlayerState;
use port::repositories::player_state_repository::PlayerStateRepository;
use crate::dto_domain_mapping::passage_mapper::passage_map_dto_to_domain;
use crate::dto_domain_mapping::player_state_mapper::{player_state_map_domain_to_dto, player_state_map_dto_to_domain};

#[derive(Clone, Debug)]
pub struct  PlayerStateQueryImpl {
    player_state_repository: Arc<dyn PlayerStateRepository>,
}

impl PlayerStateQueryImpl {
    pub fn new (player_state_repository: Arc<dyn PlayerStateRepository>) -> Self {
        PlayerStateQueryImpl {player_state_repository}
    }
}

impl PlayerStateQuery for PlayerStateQueryImpl {
    fn get_player_state(&self, bout_id: i64, player_id: i64) -> Pin<Box<dyn Future<Output=Result<Option<PlayerState>, Error>> + Send + 'static>> {
        let repo = self.player_state_repository.clone();
        Box::pin(async move {
            let player_state_dto_result = repo.find_by_bout_id_and_player_id(bout_id, player_id).await;
            player_state_dto_result
                .map(|option_dto| option_dto.map(player_state_map_dto_to_domain))
                .map_err(|e| e.into())
        })
    }

    fn persist_player_state(&self, player_state: PlayerState, bout_id: i64) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'static>> {
        let repo = self.player_state_repository.clone();
        let player_state_dto = player_state_map_domain_to_dto(& player_state);
        Box::pin(async move {
            repo.save(player_state_dto).await
                .map(|_opt_dto| ())  // Ignore the Option in the result, just map to unit type `()`.
                .map_err(|e| e.into())
        })
    }
}
