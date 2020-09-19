extern crate ctprods;
extern crate diesel;
extern crate slugify;
#[macro_use]
extern crate log;

#[macro_use]
extern crate diesel_migrations;
use dotenv::dotenv;

use env_logger;
use std::env;
use futures::stream::{ StreamExt, TryStreamExt };
use actix_multipart::{ Multipart };
use actix_web::{ http, get, put, post, web, delete, App, HttpServer, HttpResponse, middleware::Logger };
use actix_cors::Cors;
use serde_json::json;
use serde::Deserialize;
use self::ctprods::models::{ Project, NewProject, Model, NewModel, UpdatableModel, NewProjectImage, ProjectCategory, ProjectImageCategory, UpdatableProject };
use self::ctprods::middleware::auth_middleware;
use self::ctprods::establish_connection;
use self::ctprods::services::bitbucket;
use diesel_migrations::{ RunMigrationsError, embed_migrations };
use slugify::slugify;
use postgres::error::Error;
use mime;

struct AppState {}

#[get("/")]
async fn index () -> Result<HttpResponse, HttpResponse> {
    Ok(HttpResponse::MovedPermanently().header("Location", "https://www.cyprientaque.com/").await?)
}
#[get("*")]
async fn not_found_redirect () -> Result<HttpResponse, HttpResponse> {
    Ok(HttpResponse::MovedPermanently().header("Location", "https://www.cyprientaque.com/").await?)
}

#[get("/categories")]
async fn get_categories (_data: web::Data<AppState>) -> Result<HttpResponse, HttpResponse> {
    let result = ProjectCategory::all().await;
    match result {
        Ok(res) => Ok(HttpResponse::Ok().body(json!(res))),
        Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string()))
    }
}

#[get("/projects")]
async fn get_projects (_data: web::Data<AppState>) -> Result<HttpResponse, HttpResponse> {
    let result = Project::all().await;
    match result {
        Ok(res) => Ok(HttpResponse::Ok().body(json!(res))),
        Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string()))
    }
}

#[get("/projects/all_but_not_blog")]
async fn get_projects_but_not_blog (_data: web::Data<AppState>) -> Result<HttpResponse, HttpResponse> {
    let result = Project::all_but_not_blog().await;
    match result {
        Ok(res) => Ok(HttpResponse::Ok().body(json!(res))),
        Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string()))
    }
}

#[derive(Deserialize)]
struct GetProjectsByCategoryInfo{
    category_id: i32,
}
#[get("/projects/category/{category_id}")]
async fn get_projects_by_category (_data: web::Data<AppState>, info: web::Path<GetProjectsByCategoryInfo>) -> Result<HttpResponse, HttpResponse> {
    let result = Project::by_category(info.category_id).await;
    match result {
        Ok(res) => Ok(HttpResponse::Ok().body(json!(res))),
        Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string()))
    }
}

#[derive(Deserialize)]
struct GetProjectInfo{
    id: i32,
}
#[get("/projects/{id}")]
async fn get_project(_data: web::Data<AppState>, info: web::Path<GetProjectInfo>) -> Result<HttpResponse, HttpResponse> {
    let result: Result<Project, Error> = Project::find(info.id).await;

    match result {
        Ok(project) => Ok(HttpResponse::Ok().body(json!(project))),
        Err(err) => Err(HttpResponse::NotFound().body(err.to_string()))
    }
}

#[derive(Deserialize)]
struct DeleteProjectInfo{
    id: i32,
}
#[delete("/projects/{id}")]
async fn delete_project(_data: web::Data<AppState>, info: web::Path<DeleteProjectInfo>) -> Result<HttpResponse, HttpResponse> {
    let result: Result<Project, Error> = Project::find(info.id).await;

    match result {
        Ok(project) => {
            match project.delete().await {
                Ok(p) => Ok(HttpResponse::Ok().body(json!(p))),
                Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string()))
            }
        }
        Err(err) => Err(HttpResponse::NotFound().body(err.to_string()))
    }
}

#[post("/projects")]
async fn create_project(_data: web::Data<AppState>, mut new_project: web::Json<NewProject>) -> Result<HttpResponse, HttpResponse> {
    let slug: String = slugify!(&new_project.title);
    let is_unique = new_project.clone().check_slug_unique(slug.clone()).await;
    if !is_unique {
        Err(HttpResponse::BadRequest().body("Slug already used"))
    } else {
        new_project.slug = Some(slug);
        let result = new_project.clone().save().await;
        match result {
            Ok(project) => Ok(HttpResponse::Ok().body(json!(project))),
            Err(err) => Err(HttpResponse::BadRequest().body(err.to_string()))
        }
    }
}

#[put("/projects/{id}")]
async fn update_project(_data: web::Data<AppState>, info: web::Json<UpdatableProject>) -> Result<HttpResponse, HttpResponse> {

    let from_db: Result<Project, Error> = Project::find(info.id.into()).await;

    match from_db {
        Ok(_) => {
            let result = info.into_inner().update().await;
            match result {
                Ok(updated_project) => Ok(HttpResponse::Ok().body(json!(updated_project))),
                Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string()))
            }
        },
        Err(err) => Err(HttpResponse::NotFound().body(err.to_string())),
    }
}

#[derive(Deserialize)]
struct AddViewInfo{
    id: i32,
}

#[put("/projects/{id}/addView")]
async fn add_view(_data: web::Data<AppState>, info: web::Path<AddViewInfo>) -> Result<HttpResponse, HttpResponse> {

    let result: Result<Project, Error> = Project::find(info.id.into()).await;

    match result {
        Ok(mut project) => {
            project.views_count = project.views_count + 1;
            let result = project.update().await;
            match result {
                Ok(updated_project) => Ok(HttpResponse::Ok().body(json!(updated_project))),
                Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string()))
            }
        },
        Err(err) => Err(HttpResponse::NotFound().body(err.to_string())),
    }
}

#[derive(Deserialize)]
struct AddLikeInfo{
    id: i32,
}

#[put("/projects/{id}/addLike")]
async fn add_like (_data: web::Data<AppState>, info: web::Path<AddLikeInfo>) -> Result<HttpResponse, HttpResponse> {
    let result: Result<Project, Error> = Project::find(info.id.into()).await;
    match result {
        Ok(mut project) => {
            project.likes_count = project.likes_count + 1;
            let result = project.update().await;
            match result {
                Ok(updated_project) => Ok(HttpResponse::Ok().body(json!(updated_project))),
                Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string()))
            }
        },
        Err(err) => Err(HttpResponse::NotFound().body(err.to_string())),
    }
}

#[get("/projectImageCategories")]
async fn get_project_image_categories(_data: web::Data<AppState>) -> Result<HttpResponse, HttpResponse> {
    let result: Result<Vec<ProjectImageCategory>, Error> = ProjectImageCategory::all().await;
    match result {
        Ok(categories) => Ok(HttpResponse::Ok().body(json!(categories))),
        Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string()))
    }
}

#[derive(Deserialize)]
struct PostImageQuery{
    project_id: i32,
    category_id: i32,
    primary: bool
}

#[post("/projectImage")]
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
        while let Some(chunk) = field.next().await{
            let data = chunk.unwrap();
            let project_image = NewProjectImage::new(
                info.primary,
                info.project_id,
                info.category_id,
                filename.to_owned()
            );
            let image_data = data.to_vec();
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
        }
    };
    Ok(HttpResponse::Ok().into())
}


#[get("/bitbucket/accessToken")]
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

#[get("/bitbucket/refreshToken")]
async fn refresh_token (_data: web::Data<AppState>, info: web::Query<RefreshTokenQuery>) -> Result<HttpResponse, HttpResponse> {
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
                .data(AppState{})
                .service(get_projects_but_not_blog)
                .service(get_project)
                .service(get_projects_by_category)
                .service(get_projects)
                .service(create_project)
                .service(update_project)
                .service(add_view)
                .service(add_like)
                .service(delete_project)
                .service(create_project_image)
                .service(get_categories)
                .service(get_project_image_categories)
                .service(access_token)
                .service(refresh_token)
                .service(index)
                .service(not_found_redirect)
        })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
