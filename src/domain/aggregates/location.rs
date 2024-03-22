
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Location {
    pub id: i32,
    pub title: String,
    pub description: String,
    #[builder(default)]
    pub image_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_builder() {
        let location = LocationBuilder::default()
            .id(1)
            .title("Test Location Title".into())
            .description("Test Location Description".into())
            .build()
            .unwrap();

        assert_eq!(location.id, 1);
        assert_eq!(location.title, "Test Location Title");
        assert_eq!(location.description, "Test Location Description");
        assert_eq!(location.image_url, None);
    }
}