use domain_pure::model::location::Location;
use domain_pure::model::location::LocationBuilder;
use port::dto::location_dto::LocationDTO;


pub fn location_map_domain_to_dto(location: &Location) -> LocationDTO {
    LocationDTO {
        id: location.aggregate_id(),
        title: location.title().to_string(),
        description: location.description().to_string(),
        image_url: location.image_url().map(ToOwned::to_owned),
    }
}

pub fn location_map_dto_to_domain(dto: &LocationDTO) -> Location {
    LocationBuilder::default()
        .aggregate_id(dto.id)
        .title(dto.title.clone())
        .description(dto.description.clone())
        .image_url(dto.image_url.clone())
        .build()
        .expect("Failed to build Location from DTO")
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain_pure::model::location::LocationBuilder;

    #[test]
    fn test_location_to_dto_mapping() {
        let location = LocationBuilder::default()
            .aggregate_id(1)
            .title("Mysterious Forest".into())
            .description("A forest full of mysteries.".into())
            .image_url(Some("https://example.com/forest.jpg".into()))
            .build()
            .unwrap();

        let dto = location_map_domain_to_dto(&location);

        assert_eq!(dto.id, location.aggregate_id());
        assert_eq!(dto.title, location.title());
        assert_eq!(dto.description, location.description());
        assert_eq!(dto.image_url, location.image_url().map(ToOwned::to_owned));
    }

    #[test]
    fn test_dto_to_location_mapping() {
        let dto = LocationDTO {
            id: 2,
            title: "Ancient Ruins".into(),
            description: "Ruins of an ancient civilization.".into(),
            image_url: Some("https://example.com/ruins.jpg".into()),
        };

        let location = location_map_dto_to_domain(&dto);

        assert_eq!(location.aggregate_id(), dto.id);
        assert_eq!(location.title(), dto.title);
        assert_eq!(location.description(), dto.description);
        assert_eq!(location.image_url().map(ToOwned::to_owned), dto.image_url);
    }
}
