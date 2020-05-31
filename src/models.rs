use chrono::naive::NaiveDateTime;

#[derive(diesel::Queryable, serde::Serialize, serde::Deserialize)]
pub struct Project {
    pub id: i32,
    pub category_id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub views_count: i32,
    pub likes_count: i32,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub sketchfab_model_number: Option<String>,
}

#[derive(diesel::Queryable, serde::Serialize, serde::Deserialize)]
pub struct ProjectCategory {
    pub id: i32,
    pub name: String,
    pub picture_url: Option<String>,
    pub slug: String,
}

// #[derive(diesel::Queryable)]
// struct ProjectImage {
//     id: u32,
//     w1500_keyname: String,
//     w_350_keyname: String,
//     w1500_object_url: String,
//     w350_object_url: String,
//     primary: bool,
//     project_id: u32,
//     created_at: Option<NaiveDateTime>,
//     updated_at: Option<NaiveDateTime>,
// }
