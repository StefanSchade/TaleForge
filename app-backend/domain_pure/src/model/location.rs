use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, PartialEq, Clone, Debug, Deserialize, Serialize)]
#[builder(setter(into), build_fn(validate = "Self::validate"))]
pub struct Location {
    #[builder(setter(into))]
    aggregate_id: u64,
    #[builder(setter(into))]
    game_id: u64,
    #[builder(setter(into))]
    title: String,
    #[builder(setter(into))]
    description: String,
    #[builder(default, setter(into, strip_option))]
    image_url: Option<String>,
}

impl LocationBuilder {
    // Custom validation function to ensure all fields are properly initialized
    fn validate(&self) -> Result<(), String> {
        if self.aggregate_id.is_none() {
            return Err("aggregate_id must be set".to_string());
        }
        if self.game_id.is_none() {
            return Err("game_id must be set".to_string());
        }
        if self.title.is_none() {
            return Err("title must be set".to_string());
        }
        if self.description.is_none() {
            return Err("description must be set".to_string());
        }
        Ok(())
    }
}

impl Location {
    pub fn aggregate_id(&self) -> u64 {
        self.aggregate_id
    }
    pub fn game_id(&self) -> u64 {
        self.game_id
    }
    pub fn title(&self) -> &str {
        &*self.title
    }
    pub fn description(&self) -> &str {
        &*self.description
    }

    // self.image_url.as_ref() converts from &Option<String> to Option<&String>.
    // .map(AsRef::as_ref) takes each &String inside the Option and converts it to a &str,
    // resulting in an Option<&str>.
    pub fn image_url(&self) -> Option<&str> {
        self.image_url.as_ref().map(AsRef::as_ref)
    }

    // these getters return a clone that will be owned by the receiver
    pub fn title_owned(&self) -> String {
        self.title.clone()
    }
    pub fn description_owned(&self) -> String {
        self.description.clone()
    }
    pub fn image_url_owned(&self) -> Option<String> {
        self.image_url.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_builder() {
        let location = LocationBuilder::default().aggregate_id(1_u64)
            .game_id(1_u64)
            .title("Test Location Title".to_string())
            .description("Test Location Description".to_string())
            .build()
            .unwrap();

        assert_eq!(location.aggregate_id, 1);
        assert_eq!(location.title, "Test Location Title");
        assert_eq!(location.description, "Test Location Description");
        assert_eq!(location.image_url, None);
    }
}