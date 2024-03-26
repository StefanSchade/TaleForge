use domain::aggregates::location::Location;
use domain::aggregates::passage::Passage;
use port::repository::{LocationRepository, PassageRepository};
use serde_json;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

pub fn load_data_from_json<R: LocationRepository, P: PassageRepository>(
    location_repo: Arc<R>,
    passage_repo: Arc<P>,
    location_file_path: &Path,
    passage_file_path: &Path,
) -> Result<(), Box<dyn std::error::Error>>
    where
        R: LocationRepository + ?Sized,
        P: PassageRepository + ?Sized,
{
    // Load and deserialize locations
    let mut file = File::open(location_file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let locations: Vec<Location> = serde_json::from_str(&contents)?;

    println!("Locations loaded");

    // Populate the location repository
    for location in locations {
        location_repo.add_location(location)?;
    }

    // Load and deserialize passages
    let mut file = File::open(passage_file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let passages: Vec<Passage> = serde_json::from_str(&contents)?;

    // Populate the passage repository
    for passage in passages {
        passage_repo.add_passage(passage)?;
    }

    Ok(())
}
