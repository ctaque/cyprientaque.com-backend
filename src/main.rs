extern crate ctprods;
extern crate diesel;
extern crate slugify;
use dotenv::dotenv;

use actix_web::{ get, put, post, web, delete, App, HttpServer, HttpResponse };
use serde_json::json;
use serde::Deserialize;
use self::ctprods::models::{ Project, NewProject, Model, NewModel };
use slugify::slugify;
use self::ctprods::middleware::auth_middleware;
use postgres::error::Error;

struct AppState {}

#[get("/projects")]
async fn get_projects (_data: web::Data<AppState>) -> Result<HttpResponse, HttpResponse> {
    let result = Project::all().await;
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
async fn update_project(_data: web::Data<AppState>, info: web::Json<Project>) -> Result<HttpResponse, HttpResponse> {

    let result: Result<Project, Error> = Project::find(info.id.into()).await;

    match result {
        Ok(project) => {
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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(
        || App::new()
            .data(AppState{})
            .wrap(auth_middleware::Authentication)
            .service(get_project)
            .service(get_projects_by_category)
            .service(get_projects)
            .service(create_project)
            .service(update_project)
            .service(add_view)
            .service(add_like)
            .service(delete_project)
    ).bind("127.0.0.1:8088")?
        .run()
        .await
}
