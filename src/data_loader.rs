use crate::domain::aggregates::passage::Passage;
use crate::domain::aggregates::location::Location;
use crate::port::repository::{LocationRepository, PassageRepository};
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;

use crate::domain::aggregates::location::LocationBuilder;
use crate::domain::aggregates::passage::PassageBuilder;

#[derive(Deserialize)]
struct LocationData {
    // Directly map these fields to your domain model's Location struct
    id: i32,
    title: String,
    description: String,
    image_url: Option<String>,
}

#[derive(Deserialize)]
struct PassageData {
    // Directly map these fields to your domain model's Passage struct
    id: i32,
    from_location_id: i32,
    to_location_id: i32,
    description: String,
}

pub fn load_data_from_json<R: LocationRepository + 'static, P: PassageRepository + 'static>(
    location_repo: Arc<R>,
    passage_repo: Arc<P>,
    location_file_path: &Path,
    passage_file_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load and deserialize locations
    let mut file = File::open(location_file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let locations: Vec<LocationData> = serde_json::from_str(&contents)?;

    // Populate the location repository
    for location_data in locations {
        let location = LocationBuilder::default()
            .id(location_data.id)
            .title(location_data.title)
            .description(location_data.description)
            .image_url(location_data.image_url)
            .build()
            .unwrap(); // Handle this unwrap properly in production code
        location_repo.add_location(location)?; // Ensure your repository's add_location method matches this signature
    }

    // Load and deserialize passages
    let mut file = File::open(passage_file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let passages: Vec<PassageData> = serde_json::from_str(&contents)?;

    // Populate the passage repository
    for passage_data in passages {
        let passage = PassageBuilder::default()
            .id(passage_data.id)
            .from_location_id(passage_data.from_location_id)
            .to_location_id(passage_data.to_location_id)
            .description(passage_data.description)
            .build()
            .unwrap(); // Handle this unwrap properly in production code
        passage_repo.add_passage(passage)?; // Ensure your repository's add_passage method matches this signature
    }

    Ok(())
}
