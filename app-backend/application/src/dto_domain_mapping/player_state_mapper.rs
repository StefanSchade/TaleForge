use domain_pure::model::player_state::{PlayerState, PlayerStateBuilder};
use port::dto::player_state_dto::PlayerStateDTO;

pub fn location_to_dto(domain_stuct: &PlayerState) -> PlayerStateDTO {
    PlayerStateDTO {
        player_id: domain_stuct.get_player_id(),
        current_location_id: domain_stuct.current_location_id()
    }
}

pub fn dto_to_location(dto: &PlayerStateDTO) -> PlayerState {
    PlayerStateBuilder::default()
        .player_id(dto.player_id)
        .current_location_id(dto.current_location_id)
        .build()
        .expect("Failed to build PlayerState from DTO")
}

// Assuming the mapper functions are defined in `application/src/dto_domain_mapping/player_state_mapper.rs`
// This test module would be placed at the end of `player_state_mapper.rs` or in a separate file, depending on the project structure.

#[cfg(test)]
mod tests {
    use super::*; // Import the mapper functions.
use domain_pure::model::player_state::{PlayerState, PlayerStateBuilder}; // Import the PlayerState and PlayerStateBuilder.
use port::dto::player_state_dto::PlayerStateDTO; // Import the PlayerStateDTO.

    #[test]
    fn test_player_state_mapping_round_trip() {
        // Create an example PlayerState
        let original_player_state = PlayerStateBuilder::default()
            .player_id(1)
            .current_location_id(100)
            .build()
            .expect("Failed to build PlayerState");

        // Convert PlayerState to PlayerStateDTO
        let player_state_dto = location_to_dto(&original_player_state);

        // Convert PlayerStateDTO back to PlayerState
        let converted_back_player_state = dto_to_location(&player_state_dto);

        // Assert that the original PlayerState and the converted PlayerState are equivalent
        assert_eq!(original_player_state.get_player_id(), converted_back_player_state.get_player_id());
        assert_eq!(original_player_state.current_location_id(), converted_back_player_state.current_location_id());
    }

    #[test]
    fn test_player_state_dto_to_domain_default_values() {
        // Example test for default values or other edge cases
        let player_state_dto = PlayerStateDTO {
            player_id: 0, // Assume default or edge case value
            current_location_id: 0, // Assume default or edge case value
        };

        let player_state = dto_to_location(&player_state_dto);

        assert_eq!(player_state.get_player_id(), 0);
        assert_eq!(player_state.current_location_id(), 0);
    }
}
