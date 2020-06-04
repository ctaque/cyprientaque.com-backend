use chrono::naive::NaiveDateTime;
use postgres::{ Row };

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

impl User{
    pub fn new(row: &Row) -> Self{
        User{
            id: row.get("id"),
            name:row.get("name"),
            slug:row.get("slug"),
            email:row.get("email"),
            password:row.get("password"),
            punchline:row.get("punchline"),
            website_url:row.get("website_url"),
            admin:row.get("admin"),
            active:row.get("active"),
            deleted_at:row.get("deleted_at"),
            remember_token:row.get("remember_token"),
            created_at:row.get("created_at"),
            updated_at:row.get("updated_at"),
            api_token:row.get("api_token"),
        }
    }
}
