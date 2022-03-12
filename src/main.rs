extern crate ctprods;
extern crate diesel;
extern crate log;
extern crate rest_macro;
extern crate rest_macro_derive;
extern crate rust_embed;
extern crate slugify;

#[macro_use]
extern crate diesel_migrations;

use self::ctprods::command::cli::{Cmd, HandleCmd};
use self::ctprods::establish_connection;
use self::ctprods::utils::view_utils;
use actix_web::{body::Body, web, HttpResponse};
use diesel::pg::PgConnection;
use diesel_migrations::{embed_migrations, RunMigrationsError};
use dotenv::dotenv;
use handlebars::Handlebars;
use mime_guess::from_path;
use std::borrow::Cow;
use structopt::StructOpt;
use tokio;

embed_migrations!();

fn run_migrations(connection: &PgConnection) -> Result<(), RunMigrationsError> {
    embedded_migrations::run(connection)
}

#[derive(rust_embed::RustEmbed)]
#[folder = "./static"]
pub struct Asset;

fn static_files(path: web::Path<String>) -> HttpResponse {
    match Asset::get(&path.0) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            HttpResponse::Ok()
                .content_type(from_path(&path.0).first_or_octet_stream().as_ref())
                .body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

fn get_template_string(path: &str) -> String {
    String::from_utf8(Asset::get(path).unwrap().into_owned()).unwrap()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let connection = establish_connection();
    let args = Cmd::from_args();
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string(
            "blog_index",
            get_template_string("templates/blog_index.hbs"),
        )
        .unwrap();
    handlebars
        .register_template_string(
            "blog_detail",
            get_template_string("templates/blog_detail.hbs"),
        )
        .unwrap();
    handlebars
        .register_template_string("base", get_template_string("templates/partials/base.hbs"))
        .unwrap();
    handlebars.register_helper(
        "unicode_truncate",
        Box::new(view_utils::unicode_truncate_helper),
    );

    handlebars.register_helper("render_markdown", Box::new(view_utils::render_markdown));

    handlebars.register_helper("format_date", Box::new(view_utils::format_date));
    
    handlebars.register_helper(
        "format_views_count",
        Box::new(view_utils::format_views_count)
    );

    match args {
        Cmd::List => HandleCmd::list().await,
        Cmd::Create => HandleCmd::create().await,
        Cmd::AddImage { file } => HandleCmd::add_image(file).await,
        Cmd::Edit => HandleCmd::edit().await,
        Cmd::Publish => HandleCmd::publish().await,
        Cmd::Unpublish => HandleCmd::unpublish().await,
        Cmd::ChangeTitle => HandleCmd::change_title().await,
        Cmd::EditTags => HandleCmd::edit_tags().await,
        Cmd::Listen { address, port } => {
            HandleCmd::listen(
                address,
                port,
                &connection,
                run_migrations,
                handlebars,
                static_files,
            )
            .await
        }
    }
}
