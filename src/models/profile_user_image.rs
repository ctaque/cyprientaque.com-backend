use chrono::naive::NaiveDateTime;
use rest_macro::{ Model };
use async_trait::async_trait;
use postgres::{ Row, error::Error };


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ProfileUserImage{
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub user_id: i32,
    pub w1500_keyname: String,
    pub w200_keyname: String,
    pub w40_keyname: String,
    pub w1500_object_url: String,
    pub w200_object_url: String,
    pub w40_object_url: String,
}

impl ProfileUserImage{
    pub fn new(row: &Row) -> Self{
        ProfileUserImage{
            id: row.get("id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            deleted_at: row.get("deleted_at"),
            user_id: row.get("user_id"),
            w1500_keyname: row.get("w1500_keyname"),
            w200_keyname: row.get("w200_keyname"),
            w40_keyname: row.get("w40_keyname"),
            w1500_object_url: row.get("w1500_object_url"),
            w200_object_url: row.get("w200_object_url"),
            w40_object_url: row.get("w40_object_url"),
        }
    }
}

#[async_trait]
impl Model<ProfileUserImage> for ProfileUserImage {
    async fn find(id: i32) -> Result<ProfileUserImage, Error>
    where ProfileUserImage: 'async_trait{
        let row: Row = Self::db().await.query_one("select * from profile_user_images where id = $1;",  &[&id]).await?;
        let p = ProfileUserImage::new(&row);
        Ok(p)
    }
    async fn all() -> Result<Vec<ProfileUserImage>, Error>
    where ProfileUserImage: 'async_trait{
        let rows: Vec<Row> = Self::db().await.query("select * from profile_user_images;", &[]).await?;
        let mut images = Vec::new();
        for row in rows{
            let p = ProfileUserImage::new(&row);
            images.push(p);
        }
        Ok(images)
    }
    async fn update(self) -> Result<ProfileUserImage, Error>
    where ProfileUserImage: 'async_trait{
        let row: Row = Self::db().await.query_one("update profile_user_images set updated_at = CURRENT_TIMESTAMP where id = $1", &[&self.id]).await?;
        let image = ProfileUserImage::new(&row);
        Ok(image)
    }
    async fn delete(mut self) -> Result<ProfileUserImage, Error>{
        let row = Self::db().await.query_one("update profile_user_images set deleted_at = CURRENT_TIMESTAMP where id = $1", &[&self.id]).await?;
        let p = ProfileUserImage::new(&row);
        Ok(p)
    }
}
