use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Passage {
    aggregate_id: i32,
    from_location_id: i32,
    to_location_id: i32,
    description: String,
    direction: String,
    narration: String,
}

impl Passage {
    pub fn get_aggregate_id(&self) -> i32 {
        self.aggregate_id
    }

    pub fn get_from_location(&self) -> i32 {
        self.from_location_id
    }
    pub fn get_to_location(&self) -> i32 {
        self.to_location_id
    }

    // returning a string slice is more efficient since the String is not actually copied,
    // so the receiver does not get to own the String and can not change it
    pub fn _get_description_reference(&self) -> &str {
        &*self.description
    }
    pub fn get_direction_reference(&self) -> &str {
        &*self.direction
    }
    pub fn get_narration_reference(&self) -> &str {
        &*self.narration
    }

    // these getters return a clone that will be owned by the receiver
    pub fn _get_description_clone(&self) -> String {
        self.description.clone()
    }
    pub fn _get_direction_clone(&self) -> String {
        self.direction.clone()
    }
    pub fn _get_narration_clone(&self) -> String {
        self.narration.clone()
    }
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