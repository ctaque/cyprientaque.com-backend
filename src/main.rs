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
use actix_web::{ http, web, App, HttpServer, HttpResponse, middleware::Logger };
use actix_cors::Cors;
use serde_json::json;
use self::ctprods::models::{ Project, NewProject, ProjectCategory, ProjectImageCategory, UpdatableProject, ProjectImage, NewProjectImage };
use self::ctprods::middleware::auth_middleware;
use self::ctprods::establish_connection;
use self::ctprods::services::bitbucket;
use diesel_migrations::{ RunMigrationsError, embed_migrations };
use rest_macro::{HttpFind, HttpAll, HttpDelete};

async fn index () -> Result<HttpResponse, HttpResponse> {
    Ok(HttpResponse::MovedPermanently().header("Location", "https://www.cyprientaque.com/").await?)
}
async fn not_found_redirect () -> Result<HttpResponse, HttpResponse> {
    Ok(HttpResponse::MovedPermanently().header("Location", "https://www.cyprientaque.com/").await?)
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
                .route("/projects/published", web::get().to(Project::http_get_published_projects))
                .route("/projects/all_but_not_blog", web::get().to(Project::http_get_projects_but_not_blog))
                .route("/projects/{id}", web::put().to(UpdatableProject::http_update))
                .route("/projects/{id}", web::get().to(Project::http_find))
                .route("/projects/{id}", web::delete().to(Project::http_delete))
                .route("/projects/{id}/addView", web::put().to(Project::http_add_view))
                .route("/projects/{id}/addLike", web::put().to(Project::http_add_like))
                .route("/projects/{id}/publish", web::put().to(Project::http_publish_project))
                .route("/projects/{id}/unpublish", web::put().to(Project::http_unpublish_project))
                .route("/projects/category/{category_id}", web::get().to(Project::http_get_projects_by_category))
                .route("/categories", web::get().to(ProjectCategory::http_all))
                .route("/categories/{id}", web::get().to(ProjectCategory::http_find))
                .route("/categories/{id}", web::delete().to(ProjectCategory::http_delete))
                .route("/projectImageCategories", web::get().to(ProjectImageCategory::http_all))
                .route("/projectImageCategories/{id}", web::get().to(ProjectImageCategory::http_find))
                .route("/projectImageCategories/{id}", web::delete().to(ProjectImageCategory::http_delete))
                .route("/projectImage", web::get().to(ProjectImage::http_all))
                .route("/projectImage/{id}/addView", web::put().to(ProjectImage::http_add_view))
                .route("/projectImage/includeExcludeProjectCategories", web::get().to(ProjectImage::http_include_exclude_categories))
                .route("/projectImage/{id}", web::get().to(ProjectImage::http_find))
                .route("/projectImage/{id}", web::delete().to(ProjectImage::http_delete))
                .route("/projectImage", web::post().to(NewProjectImage::http_create))
                .route("/bitbucket/accessToken", web::get().to(access_token))
                .route("/bitbucket/refreshToken", web::get().to(refresh_token))
                .route("/", web::get().to(index))
                .route("*", web::get().to(not_found_redirect))
        })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
