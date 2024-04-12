use std::sync::Arc;

use domain_contract::services::navigation_services::{NavigationService, NavigationServiceTrait};

use crate::service_container::query_container::QueryContainer;

#[derive(Clone)]
pub struct DomainServiceContainer {
    navigation: Arc<dyn NavigationServiceTrait>,
}

impl DomainServiceContainer {
    pub fn new(query_container: QueryContainer) -> Self {
        Self {
            // Wrap the NavigationService in an Arc and cast it to the trait object
            navigation: Arc::new(NavigationService::new(
                query_container.location(),
                query_container.passage(),
            )) as Arc<dyn NavigationServiceTrait>,
        }
    }

    pub fn navigation(&self) -> Arc<dyn NavigationServiceTrait> {
        self.navigation.clone()
    }
}