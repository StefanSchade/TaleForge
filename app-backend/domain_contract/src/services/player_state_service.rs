use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;

use crosscutting::error_management::error::Error;
use crosscutting::error_management::standard_errors::BOUT_NOT_RUNNING;
use crosscutting::error_management::standard_errors::PLAYER_NOT_REGISTERED;
use domain_pure::model::bout::BoutStatus;
use crate::contracts::bout_query::BoutQuery;
use crate::contracts::player_state_query::PlayerStateQuery;
use domain_pure::model::player_state::{PlayerState, PlayerStateBuilder};

#[derive(Clone, Debug)]
pub struct PlayerStateService {
    bout_query: Arc<dyn BoutQuery>,
    player_state_query: Arc<dyn PlayerStateQuery>,
}

impl PlayerStateService {
    pub fn new(bout_query: Arc<dyn BoutQuery>, player_state_query: Arc<dyn PlayerStateQuery>) -> Self {
        PlayerStateService { bout_query, player_state_query }
    }

    pub fn get_or_initialize_player_state(&self, player_id: i64, bout_id: i64) -> Pin<Box<dyn Future<Output = Result<PlayerState, Error>> + Send>> {
        let bout_query = self.bout_query.clone();
        let player_state_query = self.player_state_query.clone();

        Box::pin(async move {
            let bout = bout_query.get_bout_by_id(bout_id).await?;
            if !bout.registered_participants.contains(&player_id) {
                return Err(PLAYER_NOT_REGISTERED.instantiate(vec![player_id.to_string(), bout_id.to_string()]));
            }

            if bout.status != BoutStatus::Running {
                return Err(BOUT_NOT_RUNNING.instantiate(vec![bout_id.to_string(), format!("{:?}", bout.status)]));
            }

            match player_state_query.get_player_state(player_id, bout_id).await? {
                Some(state) => Ok(state),
                None => {
                    let new_state = PlayerStateBuilder::default()
                        .player_id(player_id)
                        .bout_id(bout_id)
                        .current_location_id(1) // This should be parametrized later on
                        .build().expect("Construction of player state failed");
                    player_state_query.persist_player_state(new_state.clone(), bout_id).await?;
                    Ok(new_state)
                }
            }
        })
    }
}
