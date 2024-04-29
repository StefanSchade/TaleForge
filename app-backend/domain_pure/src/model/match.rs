use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Match {
    #[builder(setter(into))]
    aggregate_id: u64,
    #[builder(setter(into))]
    game_id: u64,
    #[builder(setter(into))]
    registered_participants: Vec<u64>,
    #[builder(setter(into))]
    status: MatchStatus,
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum MatchStatus {
    Scheduled,
    Started,
    Finished,
}