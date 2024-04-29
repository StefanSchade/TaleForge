use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoutDTO {
    pub id: u64,
    pub game_id: u64,
    pub registered_participants: Vec<u64>,
    pub status: BoutStatusDTO,
}

// Reusing the enum from the domain layer in the DTO
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum BoutStatusDTO {
    Scheduled,
    Running,
    Finished,
}