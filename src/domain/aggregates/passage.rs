#[derive(Debug, Clone)]
pub struct Passage {
    pub id: i32,
    pub from_location_id: i32,
    pub to_location_id: i32,
    pub description: String,
}