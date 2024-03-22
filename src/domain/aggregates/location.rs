
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Location {
    aggregate_id: i32,
    title: String,
    description: String,
    #[builder(default)]
    image_url: Option<String>,
}

impl Location {
    pub fn get_aggregate_id(&self) -> i32 {
        self.aggregate_id
    }

    pub fn get_title_reference(&self) -> &str {
        &*self.title
    }
    pub fn _get_description_reference(&self) -> &str {
        &*self.description
    }

    // self.image_url.as_ref() converts from &Option<String> to Option<&String>.
    // .map(AsRef::as_ref) takes each &String inside the Option and converts it to a &str,
    // resulting in an Option<&str>.
    pub fn _get_image_url_reference(&self) -> Option<&str> {
        self.image_url.as_ref().map(AsRef::as_ref)
    }

    // these getters return a clone that will be owned by the receiver
    pub fn _get_title_clone(&self) -> String {
        self.title.clone()
    }
    pub fn _get_description_clone(&self) -> String {
        self.description.clone()
    }
    pub fn _get_image_url_clone(&self) -> Option<String> {
        self.image_url.clone()
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