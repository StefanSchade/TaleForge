use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Passage {
    aggregate_id: u64,
    from_location_id: u64,
    to_location_id: u64,
    description: String,
    direction: String,
    narration: String,
}

impl Passage {
    pub fn get_aggregate_id(&self) -> u64 {
        self.aggregate_id
    }

    pub fn get_from_location(&self) -> u64 {
        self.from_location_id
    }
    pub fn get_to_location(&self) -> u64 {
        self.to_location_id
    }

    // returning a string slice is more efficient since the String is not actually copied,
    // so the receiver does not get to own the String and can not change it
    pub fn description(&self) -> &str {
        &*self.description
    }
    pub fn direction(&self) -> &str {
        &*self.direction
    }
    pub fn narration(&self) -> &str {
        &*self.narration
    }

    // these getters return a clone that will be owned by the receiver
    pub fn description_owned(&self) -> String {
        self.description.clone()
    }
    pub fn direction_owned(&self) -> String {
        self.direction.clone()
    }
    pub fn narration_owned(&self) -> String {
        self.narration.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::passage::PassageBuilder;

    #[test]
    fn test_passage_builder() {
        let passage = PassageBuilder::default()
            .aggregate_id(1)
            .from_location_id(1)
            .to_location_id(1)
            .description("Description".into())
            .direction("north".into())
            .narration("Narration".into())
            .build()
            .unwrap();

        assert_eq!(passage.aggregate_id, 1);
        assert_eq!(passage.description, "Description");
        assert_eq!(passage.direction, "north");
        assert_eq!(passage.narration, "Narration");
    }
}