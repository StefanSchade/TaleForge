use std::path::Path;
use std::sync::Arc;

use serde_json;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use application::dto_domain_mapping::location_mapper::location_map_domain_to_dto;
use application::dto_domain_mapping::passage_mapper::passage_map_domain_to_dto;
use domain_pure::model::location::Location;
use domain_pure::model::passage::Passage;
use port::repositories::location_repository::LocationRepository;
use port::repositories::passage_repository::PassageRepository;

pub async fn load_data_from_json<R, P>(
    location_repo: Arc<R>,
    passage_repo: Arc<P>,
    location_file_path: &Path,
    passage_file_path: &Path,
) -> Result<(), Box<dyn std::error::Error>>
    where
        R: LocationRepository + ?Sized,
        P: PassageRepository + ?Sized,
{
    let mut file = File::open(location_file_path).await?; // Correct use of Tokio's async file open
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let locations: Vec<Location> = serde_json::from_str(&contents)?;

    for location in locations {
        location_repo.add_location(
            location.game_id(),
            location_map_domain_to_dto(&location),
        ).await?;
    }

    let mut file = File::open(passage_file_path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let passages: Vec<Passage> = serde_json::from_str(&contents)?;

    for passage in passages {
        passage_repo.add_passage(passage_map_domain_to_dto(&passage)).await?;
    }

    Ok(())
}