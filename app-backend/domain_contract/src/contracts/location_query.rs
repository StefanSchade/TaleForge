pub mod navigation {
    use std::future::Future;
    use std::pin::Pin;
    use crosscutting::error_management::error::Error;
    use domain_pure::model::location::Location;

    pub trait LocationQueries: Send + Sync {
        fn get_location_by_aggregate_id(
            &self,
            location_aggregate_id: i32
        ) -> Pin<Box<dyn Future<Output = Result<Option<Location>, Error>> + Send>>;
    }
}
