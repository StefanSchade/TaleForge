// use std::fmt::Debug;
// use std::future::Future;
// use std::pin::Pin;
//
// use futures::future::BoxFuture;
// use futures::FutureExt;
//
// use crosscutting::error_management::error::Error;
// use domain_contract::services::navigation_services::NavigationServiceTrait;
// use domain_pure::model::location::{Location, LocationBuilder};
// use domain_pure::model::player_state::PlayerState;
// use port::dto::location_dto::LocationDTO;
// use port::dto::passage_dto::PassageDTO;
// use port::dto::player_state_dto::PlayerStateDTO;
// use port::repositories::location_repository::LocationRepository;
// use port::repositories::passage_repository::PassageRepository;
// use port::repositories::player_state_repository::PlayerStateRepository;
//
// #[derive(Debug)]
// pub(crate) struct MockForDebugLocationRepo {}
//
// impl MockForDebugLocationRepo {
//     pub(crate) fn new() -> Self {
//         Self {}
//     }
// }
//
// impl LocationRepository for MockForDebugLocationRepo {
//     fn get_location_by_id(&self, id: u64) -> BoxFuture<'static, Result<Option<LocationDTO>, Error>> {
//         async move {
//             if id == 1 {
//                 Ok(Some(LocationDTO {
//                     id,
//                     title: "Castle".to_string(),
//                     description: "A spooky old castle".to_string(),
//                     image_url: Some("http://example.com/castle.jpg".to_string()),
//                 }))
//             } else {
//                 Ok(None)
//             }
//         }.boxed()
//     }
//
//     fn get_all_locations(&self) -> BoxFuture<'static, Result<Vec<LocationDTO>, Error>> {
//         async {
//             Ok(vec![
//                 LocationDTO {
//                     id: 1,
//                     title: "Castle".to_string(),
//                     description: "A spooky old castle".to_string(),
//                     image_url: Some("http://example.com/castle.jpg".to_string()),
//                 },
//                 LocationDTO {
//                     id: 2,
//                     title: "Forest".to_string(),
//                     description: "A deep, dark wood".to_string(),
//                     image_url: None,
//                 },
//             ])
//         }.boxed()
//     }
//
//     fn add_location(&self, location: LocationDTO) -> BoxFuture<'static, Result<(), Error>> {
//         async move {
//             println!("Adding location: {:?}", location);
//             Ok(())
//         }.boxed()
//     }
// }
//
// #[derive(Debug)]
// pub(crate) struct MockForDebugPassageRepo {}
//
// impl MockForDebugPassageRepo {
//     pub(crate) fn new() -> Self {
//         Self {}
//     }
// }
//
// impl PassageRepository for MockForDebugPassageRepo {
//     fn get_passage_by_id(&self, id: u64) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>> {
//         async move {
//             // Return a mocked passage or none based on an ID
//             Ok(Some(PassageDTO {
//                 id,
//                 from_location_id: 1,
//                 to_location_id: 2,
//                 description: "A passage leading from location 1 to 2".to_string(),
//                 direction: "North".to_string(),
//                 narration: "You head north through a dark forest...".to_string(),
//             }))
//         }.boxed()
//     }
//
//     fn get_passages_for_location(&self, location_id: u64) -> BoxFuture<'static, Result<Vec<PassageDTO>, Error>> {
//         async move {
//             // Return a list of passages for a specific location
//             Ok(vec![
//                 PassageDTO {
//                     id: 1,
//                     from_location_id: location_id,
//                     to_location_id: location_id + 1,
//                     description: "A rocky path".to_string(),
//                     direction: "East".to_string(),
//                     narration: "You take a careful step on the rocky path.".to_string(),
//                 },
//                 PassageDTO {
//                     id: 2,
//                     from_location_id: location_id,
//                     to_location_id: location_id + 2,
//                     description: "A smooth road".to_string(),
//                     direction: "West".to_string(),
//                     narration: "You stride confidently on the smooth road.".to_string(),
//                 },
//             ])
//         }.boxed()
//     }
//
//     fn find_passage_by_location_and_direction(&self, location_id: u64, direction: &str) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>> {
//         let direction = direction.to_owned(); // Clone the direction into a new String
//         async move {
//             // Use the cloned direction here
//             if direction == "North" {
//                 Ok(Some(PassageDTO {
//                     id: 3,
//                     from_location_id: location_id,
//                     to_location_id: location_id + 3,
//                     description: "A northward trail".to_string(),
//                     direction, // No lifetime issues here as direction is now owned by the block
//                     narration: "You march north along a chilly trail.".to_string(),
//                 }))
//             } else {
//                 Ok(None)
//             }
//         }.boxed()
//     }
//
//     fn add_passage(&self, passage: PassageDTO) -> BoxFuture<'static, Result<(), Error>> {
//         async move {
//             println!("Adding passage: {:?}", passage);
//             Ok(())
//         }.boxed()
//     }
//
//     fn find_by_start_and_end_id(&self, from_location_id: u64, to_location_id: u64) -> BoxFuture<'static, Result<Option<PassageDTO>, Error>> {
//         async move {
//             // Mock finding a passage by start and end location
//             Ok(Some(PassageDTO {
//                 id: 4,
//                 from_location_id,
//                 to_location_id,
//                 description: "Connecting two specific locations".to_string(),
//                 direction: "Custom".to_string(),
//                 narration: "You find a hidden path connecting two landmarks.".to_string(),
//             }))
//         }.boxed()
//     }
// }
//
// #[derive(Debug)]
// pub(crate) struct MockForDebugPlayerStateRepo {}
//
// impl MockForDebugPlayerStateRepo {
//     pub(crate) fn new() -> Self {
//         Self {}
//     }
// }
//
// impl PlayerStateRepository for MockForDebugPlayerStateRepo {
//     fn find_by_player_id(&self, player_id: u64) -> BoxFuture<'static, Result<Option<PlayerStateDTO>, Error>> {
//         Box::pin(async move {
//             // Simulate fetching player state
//             Ok(Some(PlayerStateDTO {
//                 player_id,
//                 current_location_id: 1,
//             }))
//         })
//     }
//
//     fn save(&self, _: PlayerStateDTO) -> BoxFuture<'static, Result<(), Error>> {
//         Box::pin(async move {
//             Ok(())
//         })
//     }
// }
//
// #[derive(Debug)]
// pub(crate) struct MockForDebugNavigationService {}
//
// impl MockForDebugNavigationService {
//     pub(crate) fn new() -> Self {
//         Self {}
//     }
// }
//
// impl NavigationServiceTrait for MockForDebugNavigationService {
//     fn navigate(&self, _: PlayerState, _: String) -> Pin<Box<dyn Future<Output=Result<(Location, String), Error>> + Send>> {
//         Box::pin(async move {
//             // Simulate navigation result
//             Ok((LocationBuilder::default().
//                 aggregate_id(1)
//                     .description("lcc desc".to_string())
//                     .title("loc title".to_string())
//                     .build().unwrap(),
//                 "you moved".to_string()
//             ))
//         })
//     }
// }
