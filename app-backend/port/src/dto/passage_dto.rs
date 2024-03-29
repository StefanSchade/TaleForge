use serde::{Deserialize, Serialize};

#[derive!(Serialize, Deserialize, Debug)]
pub struct PassageDTO {
    pub id: i32,
    pub from_location_id: i32,
    pub to_location_id: i32,
}


// use serde::{Deserialize, Serialize};
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct LocationDTO {
//     pub id: i32,
//     pub title: String,
//     pub description: String,
//     pub image_url: Option<String>,
// }

// {
// "aggregate_id": 2,
// "from_location_id": 2,
// "to_location_id": 1,
// "description": "A winding path leading down to the old bridge.",
// "direction": "south",
// "narration": "You carefully make your way down the hill, following the winding path as it descends."
// }
// ]
