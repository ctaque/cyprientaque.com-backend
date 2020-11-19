extern crate ctprods;
extern crate diesel;
extern crate log;
extern crate rest_macro;
extern crate rest_macro_derive;
extern crate slugify;
extern crate rust_embed;

#[macro_use]
extern crate diesel_migrations;

use self::ctprods::command::cli::{Cmd, HandleCmd};
use self::ctprods::establish_connection;
use self::ctprods::utils::view_utils;
use diesel::pg::PgConnection;
use diesel_migrations::{embed_migrations, RunMigrationsError};
use dotenv::dotenv;
use handlebars::Handlebars;
use std::collections::HashMap;
use structopt::StructOpt;
use tokio;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

embed_migrations!();

fn run_migrations(connection: &PgConnection) -> Result<(), RunMigrationsError> {
    embedded_migrations::run(connection)
}

#[derive(rust_embed::RustEmbed)]
#[folder = "./static"]
struct Asset;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let connection = establish_connection();
    let args = Cmd::from_args();
    let mut handlebars = Handlebars::new();
    let blog_index = String::from_utf8(Asset::get("templates/blog_index.hbs").unwrap().into_owned()).unwrap();
    let blog_detail = String::from_utf8(Asset::get("templates/blog_detail.hbs").unwrap().into_owned()).unwrap();
    let base = String::from_utf8(Asset::get("templates/partials/base.hbs").unwrap().into_owned()).unwrap();
    handlebars
        .register_template_string(
            "blog_index",
            blog_index
        )
        .unwrap();
    handlebars
        .register_template_string(
            "blog_detail",
            blog_detail
        )
        .unwrap();
    handlebars
        .register_template_string("base", base)
        .unwrap();
    handlebars.register_helper(
        "unicode_truncate",
        Box::new(view_utils::unicode_truncate_helper),
    );

    handlebars.register_helper("render_markdown", Box::new(view_utils::render_markdown));

    handlebars.register_helper("format_date", Box::new(view_utils::format_date));

    let static_files_list = generate();

    match args {
        Cmd::List => HandleCmd::list().await,
        Cmd::Create => HandleCmd::create().await,
        Cmd::AddImage { file } => HandleCmd::add_image(file).await,
        Cmd::Edit => HandleCmd::edit().await,
        Cmd::Publish => HandleCmd::publish().await,
        Cmd::Unpublish => HandleCmd::unpublish().await,
        Cmd::Listen { address, port } => {
            HandleCmd::listen(
                address,
                port,
                &connection,
                run_migrations,
                static_files_list,
                handlebars,
            )
            .await
        }
    }
}
