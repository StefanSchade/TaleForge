use domain_pure::model::bout::{Bout, BoutStatus};
use domain_pure::model::bout::BoutBuilder;
use port::dto::bout_dto::{BoutDTO, BoutStatusDTO};

pub fn bout_map_domain_to_dto(bout: &Bout) -> BoutDTO {
    BoutDTO {
        id: bout.aggregate_id,
        game_id: bout.game_id,
        registered_participants: bout.registered_participants.clone(),
        status: match bout.status {
            BoutStatus::Scheduled => BoutStatusDTO::Scheduled,
            BoutStatus::Running => BoutStatusDTO::Running,
            BoutStatus::Finished => BoutStatusDTO::Finished,
        },
    }
}

pub fn bout_map_dto_to_domain(dto: BoutDTO) -> Result<Bout, String> {
    let status = match dto.status {
        BoutStatusDTO::Scheduled => BoutStatus::Scheduled,
        BoutStatusDTO::Running => BoutStatus::Running,
        BoutStatusDTO::Finished => BoutStatus::Finished,
    };

    BoutBuilder::default()
        .aggregate_id(dto.id)
        .game_id(dto.game_id)
        .registered_participants(dto.registered_participants)
        .status(status)
        .build()
        .map_err(|e| e.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bout_to_dto_mapping() {
        let bout = BoutBuilder::default()
            .aggregate_id(1_u64)
            .game_id(100_u64)
            .registered_participants(vec![1, 2, 3])
            .status(BoutStatus::Scheduled)
            .build()
            .unwrap();

        let dto = bout_map_domain_to_dto(&bout);

        assert_eq!(dto.id, 1);
        assert_eq!(dto.game_id, 100);
        assert_eq!(dto.registered_participants, vec![1, 2, 3]);
        assert_eq!(dto.status, BoutStatusDTO::Scheduled);
    }

    #[test]
    fn test_dto_to_bout_mapping() {
        let dto = BoutDTO {
            id: 1,
            game_id: 100,
            registered_participants: vec![1, 2, 3],
            status: BoutStatusDTO::Finished,
        };

        let bout = bout_map_dto_to_domain(dto).unwrap();

        assert_eq!(bout.aggregate_id, 1);
        assert_eq!(bout.game_id, 100);
        assert_eq!(bout.registered_participants, vec![1, 2, 3]);
        assert_eq!(bout.status, BoutStatus::Finished);
    }
}