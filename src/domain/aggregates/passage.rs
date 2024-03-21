use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
pub struct Passage {
    pub id: i32,
    pub from_location_id: i32,
    pub to_location_id: i32,
    pub description: String,
    pub direction: String, // New field for navigation direction
    pub narration: String, // New field for descriptive narration
}

#[cfg(test)]
mod tests {
    use crate::domain::aggregates::passage::PassageBuilder;
    use super::*;

    #[test]
    fn test_passage_builder() {
        let passage = PassageBuilder::default()
            .id(1)
            .from_location_id(1)
            .to_location_id(1)
            .description("Description".into())
            .direction("north".into())
            .narration("Narration".into())
            .build()
            .unwrap();

        assert_eq!(passage.id, 1);
        assert_eq!(passage.description, "Description");
        assert_eq!(passage.direction, "north");
        assert_eq!(passage.narration, "Narration");
    }
}