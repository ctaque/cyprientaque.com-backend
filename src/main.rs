extern crate ctprods;
extern crate diesel;
extern crate slugify;
extern crate rest_macro;
extern crate rest_macro_derive;
extern crate log;

#[macro_use]
extern crate diesel_migrations;

use dotenv::dotenv;
use env_logger;
use std::env;
use futures::stream::{ StreamExt, TryStreamExt };
use actix_multipart::{ Multipart };
use actix_web::{ http, web, App, HttpServer, HttpResponse, middleware::Logger };
use actix_cors::Cors;
use serde_json::json;
use serde::Deserialize;
use self::ctprods::models::{ Project, NewProject, NewProjectImage, ProjectCategory, ProjectImageCategory, UpdatableProject, ProjectImage };
use rest_macro::{Model, NewModel};
use self::ctprods::middleware::auth_middleware;
use self::ctprods::establish_connection;
use self::ctprods::services::bitbucket;
use diesel_migrations::{ RunMigrationsError, embed_migrations };
use mime;
use rest_macro::{HttpFind, HttpAll, HttpDelete};

async fn index () -> Result<HttpResponse, HttpResponse> {
    Ok(HttpResponse::MovedPermanently().header("Location", "https://www.cyprientaque.com/").await?)
}
async fn not_found_redirect () -> Result<HttpResponse, HttpResponse> {
    Ok(HttpResponse::MovedPermanently().header("Location", "https://www.cyprientaque.com/").await?)
}

#[derive(Deserialize)]
struct PostImageQuery{
    project_id: i32,
    category_id: i32,
    primary: bool
}

async fn create_project_image (mut multipart: Multipart, info: web::Query<PostImageQuery>) -> Result<HttpResponse, HttpResponse> {

    while let Ok(Some(mut field)) = multipart.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let file_mime = field.content_type();
        if file_mime.type_() != mime::IMAGE && ( file_mime.subtype() != mime::JPEG && file_mime.subtype() != mime::PNG ) {
            return Err(HttpResponse::BadRequest().body("bad mime type"));
        }
        let project = Project::find(info.project_id.into()).await;
        let category = ProjectCategory::find(info.category_id.into()).await;
        if let Err(e) = project{
            return Err(HttpResponse::BadRequest().body(e.to_string()));
        }
        if let Err(e) = category{
            return Err(HttpResponse::BadRequest().body(e.to_string()));
        }
        let filename = content_type.get_filename().unwrap();
        let mut file_stream: Vec<u8> = vec![];
        while let Some(chunk) = field.next().await{
            let data = chunk.unwrap();
            file_stream.append(&mut data.to_vec());
        }
        let project_image = NewProjectImage::new(
            info.primary,
            info.project_id,
            info.category_id,
            filename.to_owned()
        );
        let image_data = file_stream.to_vec();
        let image_350_data = &project_image.clone().generate_size(350.0, image_data.clone());
        match image_350_data {
            Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
            Ok(image) => {
                project_image.clone().upload_to_s3(&project_image.w350_keyname, image.to_vec()).await.expect("Failed uploading w350 image");
            }
        };
        let image_1500_data = &project_image.clone().generate_size(1500.0, image_data.clone());
        match image_1500_data {
            Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
            Ok(image) => {
                project_image.clone().upload_to_s3(&project_image.w1500_keyname, image.to_vec()).await.expect("Failed uploading w1500 image");
            }
        };
        match project_image.clone().upload_to_s3(&project_image.original_keyname, image_data).await {
            Ok(()) => {
                let image_save_result = project_image.save().await;

                return match image_save_result{
                    Ok(image) => Ok(HttpResponse::Ok().body(json!(image))),
                    Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string()))
                }
            }
            Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string()))
        }
    };
    Ok(HttpResponse::Ok().into())
}


async fn access_token () -> Result<HttpResponse, HttpResponse> {
    let resp = bitbucket::get_access_token().await;
    match resp{
        Ok(token) => Ok(HttpResponse::Ok().body(json!(token))),
        Err(err) => Err(HttpResponse::Unauthorized().body(err.to_string()))
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct RefreshTokenQuery{
    refresh_token: String,
}

async fn refresh_token (info: web::Query<RefreshTokenQuery>) -> Result<HttpResponse, HttpResponse> {
    let resp = bitbucket::refresh_token(info.refresh_token.to_string()).await;
    match resp{
        Ok(token) => Ok(HttpResponse::Ok().body(json!(token))),
        Err(err) => Err(HttpResponse::Unauthorized().body(err.to_string()))
    }
}

embed_migrations!();

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let connection = establish_connection();
    let migration_run = embedded_migrations::run(&connection);
    match migration_run{
        Err(e) =>
            match e {
                RunMigrationsError::MigrationError(e) => panic!(format!("Error while migrating : {}", e.to_string())),
                RunMigrationsError::QueryError(e) => panic!(format!("Error while migrating : {}", e.to_string())),
                _ => println!("Nothing to migrate"),
            },
        Ok(_) => println!("Migration successfull")
    };

    let is_prod = env::var("ENVIRONMENT").unwrap_or(String::from("development")) == String::from("production");
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    HttpServer::new(
        move || {
            App::new()
                .wrap(auth_middleware::Authentication)
                .wrap(Logger::default())
                .wrap(
                    Cors::new() // <- Construct CORS middleware builder
                        .allowed_origin(match is_prod {true => "https://www.cyprientaque.com", false => "http://localhost:3000"})
                        .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
                        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE])
                        .max_age(3600)
                        .finish())
                .app_data(web::PayloadConfig::new(900000000000000000))
                .route("/projects", web::get().to(Project::http_all))
                .route("/projects", web::post().to(NewProject::http_create))
                .route("/projects/{id}", web::put().to(UpdatableProject::http_update))
                .route("/projects/{id}", web::get().to(Project::http_find))
                .route("/projects/{id}", web::delete().to(Project::http_delete))
                .route("/projects/{id}/addView", web::put().to(Project::http_add_view))
                .route("/projects/{id}/addLike", web::put().to(Project::http_add_like))
                .route("/projects/{id}/publish", web::put().to(Project::http_publish_project))
                .route("/projects/{id}/unpublish", web::put().to(Project::http_unpublish_project))
                .route("/projects/published", web::get().to(Project::http_get_published_projects))
                .route("/projects/all_but_not_blog", web::get().to(Project::http_get_projects_but_not_blog))
                .route("/projects/category/{category_id}", web::get().to(Project::http_get_projects_by_category))
                .route("/categories", web::get().to(ProjectCategory::http_all))
                .route("/categories/{id}", web::get().to(ProjectCategory::http_find))
                .route("/categories/{id}", web::delete().to(ProjectCategory::http_delete))
                .route("/projectImageCategories", web::get().to(ProjectImageCategory::http_all))
                .route("/projectImageCategories/{id}", web::get().to(ProjectImageCategory::http_find))
                .route("/projectImageCategories/{id}", web::delete().to(ProjectImageCategory::http_delete))
                .route("/projectImage", web::get().to(ProjectImage::http_all))
                .route("/projectImage/{id}", web::get().to(ProjectImage::http_find))
                .route("/projectImage/{id}", web::delete().to(ProjectImage::http_delete))
                .route("/projectImage", web::post().to(create_project_image))
                .route("/bitbucket/accessToken", web::get().to(access_token))
                .route("/bitbucket/refreshToken", web::get().to(refresh_token))
                .route("/", web::get().to(index))
                .route("*", web::get().to(not_found_redirect))
        })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
