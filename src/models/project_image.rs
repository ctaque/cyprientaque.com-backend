use image::{self, ImageOutputFormat, DynamicImage, error::ImageResult, GenericImageView};
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
    original_object_url: Option<String>,
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
            original_object_url: row.get("original_object_url"),
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
    pub original_keyname: String,
    pub original_object_url: String,
    pub uid: String,
    pub w1500_keyname: String,
    pub w350_keyname: String,
    pub w1500_object_url: String,
    pub w350_object_url: String,
    pub primary: bool,
    pub project_image_category_id: i32,
    pub project_id: i32,
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
        let original_keyname = format!("projectsImages/{}/{}.{}", project_id, uid, ext);
        let original_object_url = format!("https://s3.{}.amazonaws.com/{}/{}", &region, &bucket, &original_keyname);
        NewProjectImage {
            original_keyname,
            original_object_url,
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

    pub async fn upload_to_s3 (self, keyname: &String, contents: Vec<u8>) -> Result<(), RusotoError<PutObjectError>> {
        let client = ConfiguredS3Client::new();
        client.put_object(keyname.to_string(), contents).await?;
        Ok(())
    }

    pub fn generate_size(self, new_w: f32, data: Vec<u8>) -> ImageResult<Vec<u8>> {
        let img = image::load_from_memory(&data)?;
        let mut result: Vec<u8> = Vec::new();

        let old_w = img.width() as f32;
        let old_h = img.height() as f32;
        let ratio = new_w.clone() / old_w;
        let new_h = (old_h * ratio).floor();

        let scaled = img.resize(new_w as u32, new_h as u32, image::imageops::FilterType::Lanczos3);
        scaled.write_to(&mut result, ImageOutputFormat::Jpeg(90)).expect("Failed to write image to result");

        Ok((*result).to_vec())
    }
}

#[async_trait]
impl NewModel<ProjectImage> for NewProjectImage {
    async fn save(self) -> Result<ProjectImage, Error>
        where ProjectImage: 'async_trait{
        if self.primary.clone(){
            Self::db().await.query(
                "update project_images set \"primary\" = false where project_id = $1",
                &[&self.project_id]
            ).await?;
        }
        let row: Row = Self::db().await.query_one(
            "insert into project_images (w1500_keyname, w350_keyname, project_image_category_id, w1500_object_url, w350_object_url, \"primary\", project_id, created_at, original_object_url) values ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP, $8) returning *;",
            &[&self.w1500_keyname, &self.w350_keyname, &self.project_image_category_id ,&self.w1500_object_url, &self.w350_object_url, &self.primary, &self.project_id, &self.original_object_url]
        ).await?;

        let image = ProjectImage::new(&row);
        Ok(image)
    }
}
