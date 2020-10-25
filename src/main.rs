extern crate ctprods;
extern crate diesel;
extern crate log;
extern crate rest_macro;
extern crate rest_macro_derive;
extern crate slugify;

#[macro_use]
extern crate diesel_migrations;

use rest_macro::{ Model, NewModel };
use self::ctprods::establish_connection;
use self::ctprods::middleware::auth_middleware;
use self::ctprods::models::{
    NewProject, NewProjectImage, Project, ProjectCategory, ProjectImage, ProjectImageCategory,
    UpdatableProject,
};
use self::ctprods::services::bitbucket;
use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web, App, HttpResponse, HttpServer};
use diesel_migrations::{embed_migrations, RunMigrationsError};
use dotenv::dotenv;
use env_logger;
use rest_macro::{HttpAll, HttpDelete, HttpFind};
use serde_json::json;
use std::env;
use structopt::StructOpt;
use clap::arg_enum;
use tokio;
use dialoguer::{ Confirm, Input, theme::ColorfulTheme, Select };
use slugify::slugify;

async fn index() -> Result<HttpResponse, HttpResponse> {
    Ok(HttpResponse::MovedPermanently()
        .header("Location", "https://www.cyprientaque.com/")
        .await?)
}
async fn not_found_redirect() -> Result<HttpResponse, HttpResponse> {
    Ok(HttpResponse::MovedPermanently()
        .header("Location", "https://www.cyprientaque.com/")
        .await?)
}

async fn access_token() -> Result<HttpResponse, HttpResponse> {
    let resp = bitbucket::get_access_token().await;
    match resp {
        Ok(token) => Ok(HttpResponse::Ok().body(json!(token))),
        Err(err) => Err(HttpResponse::Unauthorized().body(err.to_string())),
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct RefreshTokenQuery {
    refresh_token: String,
}

async fn refresh_token(info: web::Query<RefreshTokenQuery>) -> Result<HttpResponse, HttpResponse> {
    let resp = bitbucket::refresh_token(info.refresh_token.to_string()).await;
    match resp {
        Ok(token) => Ok(HttpResponse::Ok().body(json!(token))),
        Err(err) => Err(HttpResponse::Unauthorized().body(err.to_string())),
    }
}

arg_enum!{
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum Entity {
        projects,
        categories,
        image_categories,
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "ctprods", about = "cyprientaque.com/backend/cli")]
enum Cmd {
    #[structopt(name = "listen")]
    Listen {
        #[structopt(short = "p", long = "port", default_value = "8088")]
        port: String,
        #[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
        address: String,
    },
    #[structopt(name = "list")]
    List,
    #[structopt(name = "create")]
    Create,
}

embed_migrations!();

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let connection = establish_connection();

    let args = Cmd::from_args();
    match args {
        Cmd::List => {
            let entities = vec![Entity::projects, Entity::categories, Entity::image_categories];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Que voulez vous lister ?")
                .default(0)
                .items(&entities[..])
                .interact()
                .unwrap();
            let entity: &Entity = entities.get(selection).unwrap();
            let res: Result<(), String> = match entity {
                Entity::projects => Project::print_all().await,
                Entity::categories => ProjectCategory::print_all().await,
                Entity::image_categories => ProjectImageCategory::print_all().await
            };
            res.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        },
        Cmd::Create => {
            let categories: Vec<ProjectCategory> = ProjectCategory::all().await.unwrap();
            let selectified_categories: Vec<String> = ProjectCategory::selectify_categories(&categories);
            loop {
                let title = Input::<String>::new().with_prompt("Titre").interact();
                let title: String = match title {
                    Ok(title) => title,
                    Err(_) => continue
                };
                let slug: String = slugify!(&title);
                let is_unique = NewProject::check_slug_unique(slug.clone()).await;
                if !is_unique {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Slug already used".to_string()));
                }
                loop {
                    let selection = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Choisir une catégorie")
                        .default(0)
                        .items(&selectified_categories[..])
                        .interact()
                        .unwrap();
                    let selected_category: &ProjectCategory = categories.get(selection).unwrap();
                    let is_pro = Confirm::new().with_prompt("Le projet est il un projet professionnel ?").interact().unwrap();
                    let has_bitbucket_project_key = Confirm::new().with_prompt("Le projet a-t'il une clé de projet bitbucket ?").interact().unwrap();
                    let bitbucket_project_key = match has_bitbucket_project_key {
                        true =>
                            Input::<String>::new().with_prompt("Quelle est la clé du projet Bitbucket ?").interact().ok(),
                        false =>
                            Option::None
                    };
                    let mut p = NewProject {
                        title,
                        content: String::from(""),
                        category_id: selected_category.id,
                        user_id: 1,
                        sketchfab_model_number: None,
                        slug: Some(slug),
                        is_pro,
                        bitbucket_project_key,
                    };
                    let p = p.cli_edit();

                    let result: Result<Project, tokio_postgres::Error> = p.clone().save().await;
                    match result {
                        Ok(project) => {
                            println!("Success !");
                            project.pretty_print()
                        },
                        Err(err) => println!("{}", err)
                    }
                    break;
                }
                break;
            }
            Ok(())
        },
        Cmd::Listen { address, port } => {
            let migration_run = embedded_migrations::run(&connection);
            match migration_run {
                Err(e) => match e {
                    RunMigrationsError::MigrationError(e) => {
                        panic!(format!("Error while migrating : {}", e.to_string()))
                    }
                    RunMigrationsError::QueryError(e) => {
                        panic!(format!("Error while migrating : {}", e.to_string()))
                    }
                    _ => println!("Nothing to migrate"),
                },
                Ok(_) => println!("Migration successfull"),
            };

            let is_prod = env::var("ENVIRONMENT").unwrap_or(String::from("development"))
                == String::from("production");
            let mut addr = address.to_string();
            addr.push_str(":");
            addr.push_str(&port);
            env::set_var("RUST_LOG", "actix_web=debug");
            env::set_var("RUST_BACKTRACE", "full");
            env_logger::init();
            let local = tokio::task::LocalSet::new();
            let sys = actix_rt::System::run_in_tokio("server", &local);
            println!("Running server on address : {}", addr);
            let server = HttpServer::new(move || {
                App::new()
                    .wrap(auth_middleware::Authentication)
                    .wrap(Logger::default())
                    .wrap(
                        Cors::new() // <- Construct CORS middleware builder
                            .allowed_origin(match is_prod {
                                true => "https://www.cyprientaque.com",
                                false => "http://localhost:3000",
                            })
                            .allowed_methods(vec![
                                "GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS",
                            ])
                            .allowed_headers(vec![
                                http::header::AUTHORIZATION,
                                http::header::ACCEPT,
                                http::header::CONTENT_TYPE,
                            ])
                            .max_age(3600)
                            .finish(),
                    )
                    .app_data(web::PayloadConfig::new(900000000000000000))
                    .route("/projects", web::get().to(Project::http_all))
                    .route("/projects", web::post().to(NewProject::http_create))
                    .route(
                        "/projects/published",
                        web::get().to(Project::http_get_published_projects),
                    )
                    .route(
                        "/projects/all_but_not_blog",
                        web::get().to(Project::http_get_projects_but_not_blog),
                    )
                    .route(
                        "/projects/{id}",
                        web::put().to(UpdatableProject::http_update),
                    )
                    .route("/projects/{id}", web::get().to(Project::http_find))
                    .route("/projects/{id}", web::delete().to(Project::http_delete))
                    .route(
                        "/projects/{id}/addView",
                        web::put().to(Project::http_add_view),
                    )
                    .route(
                        "/projects/{id}/addLike",
                        web::put().to(Project::http_add_like),
                    )
                    .route(
                        "/projects/{id}/publish",
                        web::put().to(Project::http_publish_project),
                    )
                    .route(
                        "/projects/{id}/unpublish",
                        web::put().to(Project::http_unpublish_project),
                    )
                    .route(
                        "/projects/category/{category_id}",
                        web::get().to(Project::http_get_projects_by_category),
                    )
                    .route("/categories", web::get().to(ProjectCategory::http_all))
                    .route(
                        "/categories/{id}",
                        web::get().to(ProjectCategory::http_find),
                    )
                    .route(
                        "/categories/{id}",
                        web::delete().to(ProjectCategory::http_delete),
                    )
                    .route(
                        "/projectImageCategories",
                        web::get().to(ProjectImageCategory::http_all),
                    )
                    .route(
                        "/projectImageCategories/{id}",
                        web::get().to(ProjectImageCategory::http_find),
                    )
                    .route(
                        "/projectImageCategories/{id}",
                        web::delete().to(ProjectImageCategory::http_delete),
                    )
                    .route("/projectImage", web::get().to(ProjectImage::http_all))
                    .route(
                        "/projectImage/{id}/addView",
                        web::put().to(ProjectImage::http_add_view),
                    )
                    .route(
                        "/projectImage/includeExcludeProjectCategories",
                        web::get().to(ProjectImage::http_include_exclude_categories),
                    )
                    .route("/projectImage/{id}", web::get().to(ProjectImage::http_find))
                    .route(
                        "/projectImage/{id}",
                        web::delete().to(ProjectImage::http_delete),
                    )
                    .route(
                        "/projectImage",
                        web::post().to(NewProjectImage::http_create),
                    )
                    .route("/bitbucket/accessToken", web::get().to(access_token))
                    .route("/bitbucket/refreshToken", web::get().to(refresh_token))
                    .route("/", web::get().to(index))
                    .route("*", web::get().to(not_found_redirect))
            })
            .bind(&addr)?
            .run()
            .await?;
            sys.await?;
            Ok(server)
        }
    }
}
