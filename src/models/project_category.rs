
#[derive(diesel::Queryable, serde::Serialize, serde::Deserialize)]
pub struct ProjectCategory {
    pub id: i32,
    pub name: String,
    pub picture_url: Option<String>,
    pub slug: String,
}
