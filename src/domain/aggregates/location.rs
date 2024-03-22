
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

// visibilty set to crate to allow access to the builder
// todo: carve out the domain as a crate, so the visibility is actually restricted
#[derive(Builder, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Location {
    pub(crate) aggregate_id: i32,
    pub(crate) title: String,
    pub(crate) description: String,
    #[builder(default)]
    pub(crate) image_url: Option<String>,
}

impl Location {
    pub fn get_aggregate_id(&self) -> i32 {
        self.aggregate_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_builder() {
        let location = LocationBuilder::default().aggregate_id(1)
            .title("Test Location Title".into())
            .description("Test Location Description".into())
            .build()
            .unwrap();

        assert_eq!(location.aggregate_id, 1);
        assert_eq!(location.title, "Test Location Title");
        assert_eq!(location.description, "Test Location Description");
        assert_eq!(location.image_url, None);
    }
}