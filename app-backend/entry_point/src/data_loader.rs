use std::path::Path;
use std::sync::Arc;

use serde_json;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use application::dto_domain_mapping::location_mapper::location_map_domain_to_dto;
use application::dto_domain_mapping::passage_mapper::passage_map_domain_to_dto;
use application::dto_domain_mapping::bout_mapper::bout_map_domain_to_dto;
use domain_pure::model::location::Location;
use domain_pure::model::passage::Passage;
use domain_pure::model::bout::Bout;
use port::repositories::location_repository::LocationRepository;
use port::repositories::passage_repository::PassageRepository;
use port::repositories::bout_repository::BoutRepository;

pub async fn load_data_from_json<L, P, B>(
    location_repo: Arc<L>,
    passage_repo: Arc<P>,
    bout_repo: Arc<B>,
    location_file_path: &Path,
    passage_file_path: &Path,
    bout_file_path: &Path,
) -> Result<(), Box<dyn std::error::Error>>
    where
        L: LocationRepository + ?Sized,
        P: PassageRepository + ?Sized,
        B: BoutRepository + ?Sized,
{
    // Load and save locations
    let mut file = File::open(location_file_path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let locations: Vec<Location> = serde_json::from_str(&contents)?;

    for location in locations {
        location_repo.add_location(
            location.game_id(),
            location_map_domain_to_dto(&location),
        ).await?;
    }

    // Load and save passages
    file = File::open(passage_file_path).await?;
    contents.clear();
    file.read_to_string(&mut contents).await?;
    let passages: Vec<Passage> = serde_json::from_str(&contents)?;

    for passage in passages {
        passage_repo.add_passage(passage_map_domain_to_dto(&passage)).await?;
    }

    // Load and save bouts
    file = File::open(bout_file_path).await?;
    contents.clear();
    file.read_to_string(&mut contents).await?;
    let bouts: Vec<Bout> = serde_json::from_str(&contents)?;

    for bout in bouts {
        bout_repo.add_bout(bout_map_domain_to_dto(&bout)).await?;
    }

    Ok(())
}