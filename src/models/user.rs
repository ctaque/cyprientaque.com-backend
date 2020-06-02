use chrono::naive::NaiveDateTime;

pub struct User {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub email: String,
    pub password: String,
    pub punchline: Option<String>,
    pub website_url: Option<String>,
    pub admin: bool,
    pub active: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub remember_token: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub api_token: Option<String>
}
