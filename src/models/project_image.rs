use super::s3_client::ConfiguredS3Client;
use super::ProjectImageCategory;
use actix_web::{web, HttpResponse};
use async_trait::async_trait;
use chrono::naive::NaiveDateTime;
use image::{self, error::ImageResult, GenericImageView, ImageOutputFormat};
use postgres::{error::Error, Row};
use rest_macro::{DeleteInfo, FindInfo, HttpAll, HttpDelete, HttpFind, Model, NewModel};
use rest_macro_derive::{HttpAll, HttpDelete, HttpFind};
use rusoto_core::RusotoError;
use rusoto_s3::PutObjectError;
use serde_json::json;
use std::env;
use uuid::Uuid;
use serde::Deserialize;
use actix_multipart::{ Multipart };
use futures::stream::{ StreamExt, TryStreamExt };
use super::{ Project, ProjectCategory };
use std::fmt;


#[derive(serde::Serialize, Debug, Clone, serde::Deserialize, HttpAll, HttpFind, HttpDelete)]
pub struct ProjectImage {
    id: i32,
    w1500_keyname: String,
    w350_keyname: String,
    w1500_object_url: String,
    original_object_url: Option<String>,
    w350_object_url: String,
    primary: bool,
    views_count: i32,
    project_image_category_id: i32,
    category: Option<ProjectImageCategory>,
    project_id: i32,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
    deleted_at: Option<NaiveDateTime>,
}

impl ProjectImage {
    pub fn new(row: &Row) -> Self {
        ProjectImage {
            id: row.get("id"),
            w1500_keyname: row.get("w1500_keyname"),
            w350_keyname: row.get("w350_keyname"),
            w1500_object_url: row.get("w1500_object_url"),
            original_object_url: row.get("original_object_url"),
            project_image_category_id: row.get("project_image_category_id"),
            w350_object_url: row.get("w350_object_url"),
            primary: row.get("primary"),
            views_count: row.get("views_count"),
            project_id: row.get("project_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            deleted_at: row.get("deleted_at"),
            category: None,
        }
    }
}

#[async_trait]
impl Model<ProjectImage> for ProjectImage {
    async fn find(project_image_id: i32) -> Result<ProjectImage, Error>
    where
        ProjectImage: 'async_trait,
    {
        let row: Row = Self::db()
            .await
            .query_one(
                "select * from project_images where id = $1 and deleted_at is null;",
                &[&project_image_id],
            )
            .await?;
        let p = ProjectImage::new(&row);
        Ok(p)
    }
    async fn all() -> Result<Vec<ProjectImage>, Error>
    where
        ProjectImage: 'async_trait,
    {
        let rows: Vec<Row> = Self::db()
            .await
            .query(
                "select * from project_images where deleted_at is null;",
                &[],
            )
            .await?;
        let mut entities = Vec::new();
        for row in rows {
            let p = ProjectImage::new(&row);
            entities.push(p);
        }
        Ok(entities)
    }

    async fn update(self) -> Result<ProjectImage, Error> {
        let row: Row = Self::db().await.query_one("update project_images set \"primary\" = $2, deleted_at = $3, created_at = $4, views_count = $5, updated_at = CURRENT_TIMESTAMP where id = $1 returning *;",
                                    &[&self.id, &self.primary, &self.deleted_at, &self.created_at, &self.views_count]).await?;
        let p = ProjectImage::new(&row);
        Ok(p)
    }
    async fn delete(mut self) -> Result<ProjectImage, Error> {
        let row = Self::db()
      .await
      .query_one(
        "update project_images set deleted_at = CURRENT_TIMESTAMP where id = $1 returning *",
        &[&self.id],
      )
      .await?;
        let p = ProjectImage::new(&row);
        Ok(p)
    }
}
#[derive(serde::Deserialize)]
pub struct Id {
    pub id: i32,
}

#[derive(serde::Deserialize, Debug)]
pub struct CategoriesQuery {
    #[serde(deserialize_with = "deserialize_stringified_int_list")]
    pub categories: Vec<i32>,
    pub include: bool,
}
pub fn deserialize_stringified_int_list<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    struct IntVecVisitor;

    impl<'de> serde::de::Visitor<'de> for IntVecVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing a list of ints")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let mut ints = Vec::new();
            for id in v.replace("[", "").replace("]", "").split(",") {
                let res = id.parse::<i32>().unwrap();
                ints.push(res);
            }
            Ok(ints)
        }
    }

    deserializer.deserialize_any(IntVecVisitor)
}

impl ProjectImage{
    pub async fn http_add_view(info: web::Path<Id>) -> Result<HttpResponse, HttpResponse> {
        let ri = ProjectImage::find(info.id.into()).await;
        match ri {
            Ok(mut i) => {
                let new_primary = ProjectImage::get_image_with_max_views_for_project(i.project_id).await;
                if let Ok(new) = new_primary {
                    if new.id == i.id {
                        let reset_result = ProjectImage::reset_primary_for_project(i.project_id).await;
                        if let Ok(()) = reset_result  {
                            i.primary = true;
                        }
                    }
                }
                i.views_count = i.views_count + 1;
                let result = i.update().await;
                match result {
                    Ok(res) => Ok(HttpResponse::Ok().body(json!(res))),
                    Err(e) => Err(HttpResponse::InternalServerError().body(e.to_string()))
                }
            }
            Err(e) => Err(HttpResponse::NotFound().body(e.to_string()))
        }
    }
    pub async fn get_image_with_max_views_for_project(project_id: i32) -> Result<ProjectImage, Error>{
        let row: Row = Self::db().await.query_one("SELECT * from project_images where project_id = $1 order by views_count + 1 desc , (case when \"primary\" then 1 else 0 end) desc limit 1;", &[&project_id]).await?;
        let i = ProjectImage::new(&row);
        Ok(i)
    }

    pub async fn reset_primary_for_project(project_id: i32) -> Result<(), Error>{
        Self::db().await.query("UPDATE project_images set \"primary\" = false where project_id = $1;", &[&project_id]).await?;
        Ok(())
    }

    pub async fn http_include_exclude_categories(query: web::Query<CategoriesQuery>) ->  Result<HttpResponse, HttpResponse>{
        let mut q = String::from("SELECT i.* FROM project_images i JOIN projects p ON p.id = i.project_id WHERE p.deleted_at is null and published = true and p.category_id ");
        if query.include {
            q.push_str(" = ANY($1) and p.category_id <> 5");
        }else{
            q.push_str(" != ANY($1) and p.category_id <> 5");
        }
        let client = Self::db().await;
        let rows = client.query(&*q, &[&query.categories]).await;
        match rows {
            Ok(values) => {
                let mut res: Vec<ProjectImage> = Vec::new();
                for row in values{
                    let p = ProjectImage::new(&row);
                    let err_text = &*format!("Cannot attach project_image_category; image id: {}", &p.id);
                    let p = p.attach_category().await.expect(err_text);
                    res.push(p);
                }
                Ok(HttpResponse::Ok().body(json!(res)))

            }
            Err(e) => {
                println!("{:#?}", &e);
                Err(HttpResponse::InternalServerError().body(e.to_string()))
            }
        }
    }
    pub async fn attach_category(mut self) -> Result<ProjectImage, Error> {
        let row = Self::db()
            .await
            .query_one(
                "select * from project_image_categories where id = $1",
                &[&self.project_image_category_id],
            )
            .await?;
        let cat = ProjectImageCategory::new(&row);
        self.category = Some(cat);
        Ok(self)
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

#[derive(Deserialize)]
pub struct PostImageQuery {
    project_id: i32,
    category_id: i32,
    primary: bool,
}

impl NewProjectImage {
    pub fn new(primary: bool, project_id: i32, category_id: i32, filename: String) -> Self {
        let uid = str::replace(&Uuid::new_v4().to_string(), "-", "");
        let bucket: String = env::var("S3_BUCKET").expect("Missing S3_BUCKET in .env");
        let region: String = env::var("S3_REGION").expect("Missing S3_REGION in .env");
        let ext: &str = filename.split(".").collect::<Vec<&str>>().last().unwrap();
        let w1500_keyname = format!("projectsImages/{}/w1500-{}.{}", project_id, uid, ext);
        let w350_keyname = format!("projectsImages/{}/w350-{}.{}", project_id, uid, ext);
        let w1500_object_url = format!(
            "https://s3.{}.amazonaws.com/{}/{}",
            &region, &bucket, &w1500_keyname
        );
        let w350_object_url = format!(
            "https://s3.{}.amazonaws.com/{}/{}",
            &region, &bucket, &w350_keyname
        );
        let original_keyname = format!("projectsImages/{}/{}.{}", project_id, uid, ext);
        let original_object_url = format!(
            "https://s3.{}.amazonaws.com/{}/{}",
            &region, &bucket, &original_keyname
        );
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

    pub async fn http_create(
        mut multipart: Multipart,
        info: web::Query<PostImageQuery>,
    ) -> Result<HttpResponse, HttpResponse> {
        while let Ok(Some(mut field)) = multipart.try_next().await {
            let content_type = field.content_disposition().unwrap();
            let file_mime = field.content_type();
            if file_mime.type_() != mime::IMAGE
                && (file_mime.subtype() != mime::JPEG && file_mime.subtype() != mime::PNG)
            {
                return Err(HttpResponse::BadRequest().body("bad mime type"));
            }
            let project = Project::find(info.project_id.into()).await;
            let category = ProjectCategory::find(info.category_id.into()).await;
            if let Err(e) = project {
                return Err(HttpResponse::BadRequest().body(e.to_string()));
            }
            if let Err(e) = category {
                return Err(HttpResponse::BadRequest().body(e.to_string()));
            }
            let filename = content_type.get_filename().unwrap();
            let mut file_stream: Vec<u8> = vec![];
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                file_stream.append(&mut data.to_vec());
            }
            let project_image = NewProjectImage::new(
                info.primary,
                info.project_id,
                info.category_id,
                filename.to_owned(),
            );
            let image_data = file_stream.to_vec();
            let image_350_data = &project_image
                .clone()
                .generate_size(350.0, image_data.clone());
            match image_350_data {
                Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
                Ok(image) => {
                    project_image
                        .clone()
                        .upload_to_s3(&project_image.w350_keyname, image.to_vec())
                        .await
                        .expect("Failed uploading w350 image");
                }
            };
            let image_1500_data = &project_image
                .clone()
                .generate_size(1500.0, image_data.clone());
            match image_1500_data {
                Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
                Ok(image) => {
                    project_image
                        .clone()
                        .upload_to_s3(&project_image.w1500_keyname, image.to_vec())
                        .await
                        .expect("Failed uploading w1500 image");
                }
            };
            match project_image
                .clone()
                .upload_to_s3(&project_image.original_keyname, image_data)
                .await
            {
                Ok(()) => {
                    let image_save_result = project_image.save().await;

                    return match image_save_result {
                        Ok(image) => Ok(HttpResponse::Ok().body(json!(image))),
                        Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
                    };
                }
                Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
            }
        }
        Ok(HttpResponse::Ok().into())
    }

    pub async fn upload_to_s3(
        self,
        keyname: &String,
        contents: Vec<u8>,
    ) -> Result<(), RusotoError<PutObjectError>> {
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

        let scaled = img.resize(
            new_w as u32,
            new_h as u32,
            image::imageops::FilterType::Lanczos3,
        );
        scaled
            .write_to(&mut result, ImageOutputFormat::Jpeg(90))
            .expect("Failed to write image to result");

        Ok((*result).to_vec())
    }
}

#[async_trait]
impl NewModel<ProjectImage> for NewProjectImage {
    async fn save(self) -> Result<ProjectImage, Error>
    where
        ProjectImage: 'async_trait,
    {
        if self.primary.clone() {
            Self::db()
                .await
                .query(
                    "update project_images set \"primary\" = false where project_id = $1",
                    &[&self.project_id],
                )
                .await?;
        }
        let row: Row = Self::db().await.query_one(
            "insert into project_images (w1500_keyname, w350_keyname, project_image_category_id, w1500_object_url, w350_object_url, \"primary\", project_id, created_at, original_object_url) values ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP, $8) returning *;",
            &[&self.w1500_keyname, &self.w350_keyname, &self.project_image_category_id ,&self.w1500_object_url, &self.w350_object_url, &self.primary, &self.project_id, &self.original_object_url]
        ).await?;

        let image = ProjectImage::new(&row);
        Ok(image)
    }
}
