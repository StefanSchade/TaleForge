use domain_pure::model::player_state::{PlayerState, PlayerStateBuilder};
use port::dto::player_state_dto::PlayerStateDTO;

pub fn player_state_map_domain_to_dto(domain: &PlayerState) -> PlayerStateDTO {
    PlayerStateDTO {
        player_id: domain.player_id(),
        bout_id: domain.bout_id(),
        current_location_id: domain.current_location_id(),
    }
}

pub fn player_state_map_dto_to_domain(dto: PlayerStateDTO) -> PlayerState {
    PlayerStateBuilder::default()
        .player_id(dto.player_id)
        .bout_id(dto.bout_id)
        .current_location_id(dto.current_location_id)
        .build()
        .expect("Failed to build PlayerState from DTO")
}

#[cfg(test)]
mod tests {
    use domain_pure::model::player_state::PlayerStateBuilder;
    use port::dto::player_state_dto::PlayerStateDTO;

    use super::*;

    #[test]
    fn test_player_state_mapping_round_trip() {
        let original_player_state = PlayerStateBuilder::default()
            .player_id(1_u64)
            .bout_id(1_u64)
            .current_location_id(100_u64)
            .build()
            .expect("Failed to build PlayerState");

        let player_state_dto = player_state_map_domain_to_dto(&original_player_state);
        let converted_back_player_state = player_state_map_dto_to_domain(player_state_dto);

        assert_eq!(original_player_state.player_id(), converted_back_player_state.player_id());
        assert_eq!(original_player_state.current_location_id(), converted_back_player_state.current_location_id());
    }


    #[test]
    fn test_player_state_dto_to_domain_default_values() {
        let player_state_dto = PlayerStateDTO {
            player_id: 0,
            bout_id: 1,
            current_location_id: 0,
        };

        let player_state = player_state_map_dto_to_domain(player_state_dto);

        assert_eq!(player_state.player_id(), 0);
        assert_eq!(player_state.current_location_id(), 0);
    }
}