use domain_pure::model::location::Location;
use domain_pure::model::location::LocationBuilder;
use port::dto::location_dto::LocationDTO;

pub fn location_map_domain_to_dto(location: &Location) -> LocationDTO {
    LocationDTO {
        id: location.aggregate_id(),
        game_id: location.game_id(),
        title: location.title_owned(),
        description: location.description_owned(),
        image_url: location.image_url_owned(),
    }
}

pub fn location_map_dto_to_domain(dto: LocationDTO) -> Location {
    LocationBuilder::default()
        .aggregate_id(dto.id)
        .game_id(dto.game_id)
        .title(dto.title)
        .description(dto.description)
        // the domain does expect an option, but the builder does not due to the 'strip_option' annotation
        .image_url(dto.image_url.unwrap_or_default())
        .build()
        .expect("Failed to build Location from DTO")
}

#[cfg(test)]
mod tests {
    use domain_pure::model::location::LocationBuilder;

    use super::*;

    #[test]
    fn test_location_to_dto_mapping() {
        let location = LocationBuilder::default()
            .aggregate_id(1_u64)
            .game_id(1_u64)
            .title("Mysterious Forest".to_string())
            .description("A forest full of mysteries.".to_string())
            .image_url("https://example.com/forest.jpg".to_string())
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
            game_id: 1,
            title: "Ancient Ruins".into(),
            description: "Ruins of an ancient civilization.".into(),
            image_url: Some("https://example.com/ruins.jpg".into()),
        };

        // Clone the data needed for comparison
        let id_clone = dto.id;
        let title_clone = dto.title.clone();
        let description_clone = dto.description.clone();
        let image_url_clone = dto.image_url.clone();

        let location = location_map_dto_to_domain(dto); // dto is moved here

        assert_eq!(location.aggregate_id(), id_clone);
        assert_eq!(location.title(), title_clone);
        assert_eq!(location.description(), description_clone);
        assert_eq!(location.image_url_owned(), image_url_clone);
    }
}