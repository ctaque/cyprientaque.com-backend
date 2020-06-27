use chrono::naive::NaiveDateTime;
use postgres::{ Row, error::Error };
use async_trait::async_trait;
use crate::models::{ ProfileUserImage, model::{ Model } };

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
    pub api_token: Option<String>,
    pub profile_images: Option<Vec<ProfileUserImage>>
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
            profile_images: Option::None,
        }
    }
    pub async fn attach_profile_images(mut self) -> Result<User, Error>{
        let rows = Self::db().await.query("select * from profile_user_images where user_id = $1", &[&self.id]).await?;
        let mut images = Vec::new();
        for row in rows{
            let image = ProfileUserImage::new(&row);
            images.push(image);
        }
        self.profile_images = Some(images);
        Ok(self)
    }
}

#[async_trait]
impl Model<User> for User {
    async fn find(id: i32) -> Result<User, Error>
    where User: 'async_trait{
        let row: Row = Self::db().await.query_one("select * from users where id = $1 and deleted_at is null;",  &[&id]).await?;
        let u = User::new(&row);
        Ok(u)
    }
    async fn all() -> Result<Vec<User>, Error>
    where User: 'async_trait{
        let rows: Vec<Row> = Self::db().await.query("select * from users where deleted_at is null;", &[]).await?;
        let mut users = Vec::new();
        for row in rows{
            let u = User::new(&row);
            users.push(u);
        }
        Ok(users)
    }
    async fn update(self) -> Result<User, Error> {

        let row: Row = Self::db().await.query_one("update users set name = $2, slug = $3, email = $4, password = $5, punchline = $6, website_url = $7, admin = $8, active = $9, remember_token = $10, created_at = $11, updated_at = CURRENT_TIMESTAMP, api_token = $12 where id = $1 returning *;",
                                    &[&self.id, &self.name, &self.slug, &self.email, &self.password, &self.punchline, &self.website_url, &self.admin, &self.active, &self.remember_token, &self.created_at, &self.api_token]).await?;
        let p = User::new(&row);
        Ok(p)

    }
    async fn delete(mut self) -> Result<User, Error>{
        let row = Self::db().await.query_one("update projects set deleted_at = CURRENT_TIMESTAMP where id = $1", &[&self.id]).await?;
        let u = User::new(&row);
        Ok(u)
    }
}
