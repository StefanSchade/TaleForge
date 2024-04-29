use domain_pure::model::passage::{Passage, PassageBuilder};
use port::dto::passage_dto::PassageDTO;

pub fn passage_map_domain_to_dto(passage: &Passage) -> PassageDTO {
    PassageDTO {
        id: passage.get_aggregate_id(),
        from_location_id: passage.get_from_location(),
        to_location_id: passage.get_to_location(),
        description: passage.description_owned(),
        direction: passage.direction_owned(),
        narration: passage.narration_owned(),
    }
}

pub fn passage_map_dto_to_domain(dto: PassageDTO) -> Passage {
    PassageBuilder::default()
        .aggregate_id(dto.id)
        .from_location_id(dto.from_location_id)
        .to_location_id(dto.to_location_id)
        .description(dto.description)
        .direction(dto.direction)
        .narration(dto.narration)
        .build()
        .expect("Failed to build Passage from dto")
}

#[cfg(test)]
mod tests {
    use domain_pure::model::passage::PassageBuilder;

    use super::*;

    #[test]
    fn test_player_state_mapping_round_trip() {
        let original_passage = PassageBuilder::default()
            .aggregate_id(1)
            .from_location_id(1)
            .to_location_id(2)
            .description("desc".to_string())
            .narration("narr".to_string())
            .direction("dir".to_string())
            .build()
            .expect("Failed to build Passage");

        let passage_dto = passage_map_domain_to_dto(&original_passage);
        let converted_back_player_state = passage_map_dto_to_domain(passage_dto);

        assert_eq!(original_passage.get_aggregate_id(), converted_back_player_state.get_aggregate_id());
        assert_eq!(original_passage.get_to_location(), converted_back_player_state.get_to_location());
        assert_eq!(original_passage.get_from_location(), converted_back_player_state.get_from_location());
        assert_eq!(original_passage.description_owned(), converted_back_player_state.description_owned());
        assert_eq!(original_passage.narration_owned(), converted_back_player_state.narration_owned());
        assert_eq!(original_passage.direction_owned(), converted_back_player_state.direction_owned());
    }
}