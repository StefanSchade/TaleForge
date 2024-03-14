#[derive(Debug, Clone)]
pub struct Location {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
}