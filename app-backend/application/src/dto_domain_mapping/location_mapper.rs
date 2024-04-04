use domain_pure::model::location::Location;
use domain_pure::model::location::LocationBuilder;
use port::dto::location_dto::LocationDTO;


pub fn location_to_dto(location: &Location) -> LocationDTO {
    LocationDTO {
        id: location.get_aggregate_id(),
        title: location.get_title_reference().to_string(),
        description: location._get_description_reference().to_string(),
        image_url: location._get_image_url_reference().map(ToOwned::to_owned),
    }
}

pub fn dto_to_location(dto: &LocationDTO) -> Location {
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
    use domain_pure::model::location::LocationBuilder; // Adjust this path based on your actual module structure.

    #[test]
    fn test_location_to_dto_mapping() {
        let location = LocationBuilder::default()
            .aggregate_id(1)
            .title("Mysterious Forest".into())
            .description("A forest full of mysteries.".into())
            .image_url(Some("https://example.com/forest.jpg".into()))
            .build()
            .unwrap();

        let dto = location_to_dto(&location);

        assert_eq!(dto.id, location.get_aggregate_id());
        assert_eq!(dto.title, location.get_title_reference());
        assert_eq!(dto.description, location._get_description_reference());
        assert_eq!(dto.image_url, location._get_image_url_reference().map(ToOwned::to_owned));
    }

    #[test]
    fn test_dto_to_location_mapping() {
        let dto = LocationDTO {
            id: 2,
            title: "Ancient Ruins".into(),
            description: "Ruins of an ancient civilization.".into(),
            image_url: Some("https://example.com/ruins.jpg".into()),
        };

        let location = dto_to_location(&dto);

        assert_eq!(location.get_aggregate_id(), dto.id);
        assert_eq!(location.get_title_reference(), dto.title);
        assert_eq!(location._get_description_reference(), dto.description);
        assert_eq!(location._get_image_url_reference().map(ToOwned::to_owned), dto.image_url);
    }
}
