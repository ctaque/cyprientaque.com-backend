use chrono::naive::NaiveDateTime;
use postgres::{ Row };

#[derive(serde::Serialize, Debug, Clone, serde::Deserialize)]
pub struct ProjectImage {
    id: i32,
    w1500_keyname: String,
    w350_keyname: String,
    w1500_object_url: String,
    w350_object_url: String,
    primary: bool,
    project_id: i32,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

impl ProjectImage {
    pub fn new(row: &Row) -> Self{
        ProjectImage{
            id: row.get("id"),
            w1500_keyname: row.get("w1500_keyname"),
            w350_keyname: row.get("w350_keyname"),
            w1500_object_url: row.get("w1500_object_url"),
            w350_object_url: row.get("w350_object_url"),
            primary: row.get("primary"),
            project_id: row.get("project_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),

        }
    }
}
