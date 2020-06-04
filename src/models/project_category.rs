use postgres::{ Row };

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ProjectCategory {
    pub id: i32,
    pub name: String,
    pub picture_url: Option<String>,
    pub slug: String,
}

impl ProjectCategory{
    pub fn new(row: &Row) -> Self{
        ProjectCategory{
            id: row.get("id"),
            name: row.get("name"),
            picture_url: row.get("picture_url"),
            slug: row.get("slug")
        }
    }
}
