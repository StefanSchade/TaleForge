pub mod navigation {
    use std::fmt::Debug;
    use std::future::Future;
    use std::pin::Pin;

    use crosscutting::error_management::error::Error;
    use domain_pure::model::location::Location;

    pub trait LocationQueries: Send + Sync + Debug {
        fn get_location_by_game_id_and_aggregate_id(
            &self,
            game_id: i64,
            location_aggregate_id: i64,
        ) -> Pin<Box<dyn Future<Output=Result<Option<Location>, Error>> + Send>>;
    }
}