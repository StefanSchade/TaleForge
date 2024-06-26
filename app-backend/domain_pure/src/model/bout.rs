use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Bout {
    #[builder(setter(into))]
    pub aggregate_id: i64,
    #[builder(setter(into))]
    pub game_id: i64,
    #[builder(setter(into))]
    pub registered_participants: Vec<i64>,
    #[builder(setter(into))]
    pub status: BoutStatus,
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum BoutStatus {
    Scheduled,
    Running,
    Finished,
}

impl Bout {
    pub fn get_game_id_for_bout(&self) -> i64 {
        self.game_id
    }
}