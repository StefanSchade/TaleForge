use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Passage {
    pub (crate) aggregate_root: i32,
    pub (crate) from_location_id: i32,
    pub (crate) to_location_id: i32,
    pub (crate) description: String,
    pub (crate) direction: String,
    pub (crate) narration: String,
}

#[cfg(test)]
mod tests {
    use crate::domain::aggregates::passage::PassageBuilder;

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

        assert_eq!(passage.aggregate_root, 1);
        assert_eq!(passage.description, "Description");
        assert_eq!(passage.direction, "north");
        assert_eq!(passage.narration, "Narration");
    }
}