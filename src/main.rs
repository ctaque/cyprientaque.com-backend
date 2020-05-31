use actix_web::{get, App, HttpServer, Responder};

#[get("/index.html")]
async fn index() -> impl Responder {
    format!("Hello World {}!", "yeah")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
