extern crate ctprods;
extern crate diesel;
extern crate slugify;

use actix_web::{ get, put, post, web, delete, App, HttpServer, HttpResponse };
use serde_json::json;
use serde::Deserialize;
use self::ctprods::establish_connection;
use self::diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;
use self::ctprods::models::{ Project, NewProject, Model, NewModel };
use slugify::slugify;

struct AppState {
    db: PgConnection
}

#[get("/projects")]
async fn get_projects (data: web::Data<AppState>) -> Result<HttpResponse, HttpResponse> {
    let result = Project::all(&data.db);
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
async fn get_projects_by_category (data: web::Data<AppState>, info: web::Path<GetProjectsByCategoryInfo>) -> Result<HttpResponse, HttpResponse> {
    let result = Project::by_category(&data.db, info.category_id);
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
async fn get_project(data: web::Data<AppState>, info: web::Path<GetProjectInfo>) -> Result<HttpResponse, HttpResponse> {
    let result: Result<Project, Error> = Project::find(&data.db, info.id);

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
async fn delete_project(data: web::Data<AppState>, info: web::Path<DeleteProjectInfo>) -> Result<HttpResponse, HttpResponse> {
    let result: Result<Project, Error> = Project::find(&data.db, info.id);

    match result {
        Ok(project) => {
            match project.delete(&data.db) {
                Ok(p) => Ok(HttpResponse::Ok().body(json!(p))),
                Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string()))
            }
        }
        Err(err) => Err(HttpResponse::NotFound().body(err.to_string()))
    }
}

#[post("/projects")]
async fn create_project(data: web::Data<AppState>, mut new_project: web::Json<NewProject>) -> Result<HttpResponse, HttpResponse> {
    let slug: String = slugify!(&new_project.title);
    let is_unique = new_project.clone().check_slug_unique(slug.clone(), &data.db);
    if !is_unique {
        Err(HttpResponse::BadRequest().body("Slug already used"))
    } else {
        new_project.slug = Some(slug);
        let result = new_project.clone().save(&data.db);
        match result {
            Ok(project) => Ok(HttpResponse::Ok().body(json!(project))),
            Err(err) => Err(HttpResponse::BadRequest().body(err.to_string()))
        }
    }
}

#[derive(Deserialize)]
struct AddViewInfo{
    id: i32,
}

#[put("/projects/{id}/addView")]
async fn add_view(data: web::Data<AppState>, info: web::Path<AddViewInfo>) -> Result<HttpResponse, HttpResponse> {
    use ctprods::schema::projects::dsl::{ projects,  deleted_at };

    let result: Result<Project, Error> = projects.filter(deleted_at.is_null()).find::<i32>(info.id.into()).first(&data.db);

    match result {
        Ok(mut project) => {
            project.views_count = project.views_count + 1;
            let result = project.update(&data.db);
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
async fn add_like (data: web::Data<AppState>, info: web::Path<AddLikeInfo>) -> Result<HttpResponse, HttpResponse> {
    use ctprods::schema::projects::dsl::{ projects, deleted_at };
    let result: Result<Project, Error> = projects.filter(deleted_at.is_null()).find::<i32>(info.id.into()).first(&data.db);
    match result {
        Ok(mut project) => {
            project.likes_count = project.likes_count + 1;
            let result = project.update(&data.db);
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
    HttpServer::new(
        || App::new().data(AppState{db: establish_connection()}
        ).service(get_project)
            .service(get_projects_by_category)
            .service(get_projects)
            .service(create_project)
            .service(add_view)
            .service(add_like)
            .service(delete_project)
    ).bind("127.0.0.1:8088")?
        .run()
        .await
}
