use chrono::naive::NaiveDateTime;
use async_trait::async_trait;
use postgres::{ Row, error::Error };
use std::env;
use crate::models::{ model::{ NewModel }, s3_client::ConfiguredS3Client };
use rusoto_s3::PutObjectError;
use rusoto_core::{ RusotoError};
use uuid::Uuid;


#[derive(serde::Serialize, Debug, Clone, serde::Deserialize)]
pub struct ProjectImage {
    id: i32,
    w1500_keyname: String,
    w350_keyname: String,
    w1500_object_url: String,
    w350_object_url: String,
    primary: bool,
    project_image_category_id: i32,
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
            project_image_category_id: row.get("project_image_category_id"),
            w350_object_url: row.get("w350_object_url"),
            primary: row.get("primary"),
            project_id: row.get("project_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),

        }
    }
}

#[derive(serde::Serialize, Debug, Clone, serde::Deserialize)]
pub struct NewProjectImage {
    original_path: String,
    uid: String,
    w1500_keyname: String,
    w350_keyname: String,
    w1500_object_url: String,
    w350_object_url: String,
    primary: bool,
    project_image_category_id: i32,
    project_id: i32,
}

impl NewProjectImage {
    pub fn new(primary: bool, project_id: i32, category_id: i32, filename: String) -> Self {
        let uid = str::replace(&Uuid::new_v4().to_string(), "-", "");
        let bucket: String = env::var("S3_BUCKET").expect("Missing S3_BUCKET in .env");
        let region: String = env::var("S3_REGION").expect("Missing S3_REGION in .env");
        let ext: &str = filename.split(".").collect::<Vec<&str>>().last().unwrap();
        let w1500_keyname = format!("projectsImages/{}/w1500-{}.{}", project_id, uid, ext);
        let w350_keyname = format!("projectsImages/{}/w350-{}.{}", project_id, uid, ext);
        let w1500_object_url = format!("https://s3.{}.amazonaws.com/{}/{}", &region, &bucket, &w1500_keyname);
        let w350_object_url = format!("https://s3.{}.amazonaws.com/{}/{}", &region, &bucket, &w350_keyname);
        let original_path = format!("projectsImages/{}/{}.{}", project_id, uid, ext);
        NewProjectImage {
            original_path,
            uid,
            w1500_keyname,
            w350_keyname,
            w1500_object_url,
            w350_object_url,
            primary,
            project_image_category_id: category_id,
            project_id,
        }
    }

    pub async fn upload_to_s3 (self, contents: Vec<u8>) -> Result<(), RusotoError<PutObjectError>> {
        let client = ConfiguredS3Client::new();
        client.put_object(self.original_path, contents).await?;
        Ok(())
    }
}

#[async_trait]
impl NewModel<ProjectImage> for NewProjectImage {
    async fn save(self) -> Result<ProjectImage, Error>
        where ProjectImage: 'async_trait{
        println!("{:#?}", &self);
        let row: Row = Self::db().await.query_one(
            "insert into project_images (w1500_keyname, w350_keyname, project_image_category_id, w1500_object_url, w350_object_url, \"primary\", project_id, created_at) values ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP) returning *;",
            &[&self.w1500_keyname, &self.w350_keyname, &self.project_image_category_id ,&self.w1500_object_url, &self.w350_object_url, &self.primary, &self.project_id]
        ).await?;

        let image = ProjectImage::new(&row);
        Ok(image)
    }
}
