extern crate ctprods;
extern crate diesel;

use actix_web::{ get, put, web, App, HttpServer, HttpResponse };
use serde_json::json;
use serde::Deserialize;
use self::ctprods::establish_connection;
use self::diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;
use self::ctprods::models::Project;

struct AppState {
    db: PgConnection
}
#[derive(Deserialize)]
struct GetProjectInfo{
    id: i32,
}
#[get("/projects/{id}")]
async fn get_project(data: web::Data<AppState>, info: web::Path<GetProjectInfo>) -> Result<HttpResponse, HttpResponse> {
    use ctprods::schema::projects::dsl::{ projects, deleted_at };
    let result: Result<Project, Error> = projects.filter(deleted_at.is_null()).find::<i32>(info.id.into()).first(&data.db);

    match result {
        Ok(project) => Ok(HttpResponse::Ok().body(json!(project))),
        Err(err) => match err {
            Error::NotFound => Err(HttpResponse::NotFound().body(err.to_string())),
            _ => Err(HttpResponse::InternalServerError().body(err.to_string()))
        }
    }
}

#[derive(Deserialize)]
struct AddViewInfo{
    id: i32,
}

#[put("/projects/{id}/addView")]
async fn add_view(data: web::Data<AppState>, info: web::Path<AddViewInfo>) -> Result<HttpResponse, HttpResponse> {
    use ctprods::schema::projects::dsl::{ projects, id,  deleted_at, views_count };

    let result: Result<Project, Error> = projects.filter(deleted_at.is_null()).find::<i32>(info.id.into()).first(&data.db);

    match result {
        Ok(mut project) => {
            project.views_count = project.views_count + 1;
            let updated_row: Result<Project, Error> = diesel::update(projects.filter(id.eq(info.id)).or_filter(deleted_at.is_null()))
                .set(views_count.eq(project.views_count))
                .get_result(&data.db);
            match updated_row {
                Ok(updated_project) => Ok(HttpResponse::Ok().body(json!(updated_project))),
                Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string()))
            }
        },
        Err(err) => match err {
            Error::NotFound => Err(HttpResponse::NotFound().body(err.to_string())),
            _ => Err(HttpResponse::InternalServerError().body(err.to_string()))
        }
    }
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new().data(AppState{db: establish_connection()}
        ).service(get_project)
            .service(add_view)
    ).bind("127.0.0.1:8088")?
        .run()
        .await
}
