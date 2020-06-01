extern crate ctprods;
extern crate diesel;

use actix_web::{ get, web, App, HttpServer, HttpResponse };
use serde_json::json;
use self::ctprods::establish_connection;
use self::diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;
use self::ctprods::models::Project;

struct AppState {
    db: PgConnection
}

#[derive(serde::Deserialize)]
struct GetProjectInfo{
    id: i32,
}
#[get("/projects/{id}")]
async fn index(data: web::Data<AppState>, info: web::Path<GetProjectInfo>) -> Result<HttpResponse, HttpResponse> {
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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new().data(AppState{db: establish_connection()}).service(index)
    ).bind("127.0.0.1:8088")?
        .run()
        .await
}
