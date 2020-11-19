extern crate ctprods;
extern crate diesel;
extern crate log;
extern crate rest_macro;
extern crate rest_macro_derive;
extern crate slugify;

#[macro_use]
extern crate diesel_migrations;

use self::ctprods::command::cli::{Cmd, HandleCmd};
use self::ctprods::establish_connection;
use self::ctprods::utils::view_utils;
use diesel::pg::PgConnection;
use diesel_migrations::{embed_migrations, RunMigrationsError};
use dotenv::dotenv;
use handlebars::Handlebars;
use structopt::StructOpt;
use tokio;
use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

embed_migrations!();

fn run_migrations(connection: &PgConnection) -> Result<(), RunMigrationsError> {
    embedded_migrations::run(connection)
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let connection = establish_connection();
    let args = Cmd::from_args();

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("blog_index", "./static/templates/blog_index.hbs")
        .unwrap();
    handlebars
        .register_template_file("blog_detail", "./static/templates/blog_detail.hbs")
        .unwrap();
    handlebars
        .register_template_file("base", "./static/templates/partials/base.hbs")
        .unwrap();
    handlebars
        .register_helper("unicode_truncate", Box::new(view_utils::unicode_truncate_helper));

    handlebars
        .register_helper("render_markdown", Box::new(view_utils::render_markdown));

    handlebars
        .register_helper("format_date", Box::new(view_utils::format_date));

    let static_files_list = generate();

    match args {
        Cmd::List => HandleCmd::list().await,
        Cmd::Create => HandleCmd::create().await,
        Cmd::AddImage { file } => HandleCmd::add_image(file).await,
        Cmd::Edit => HandleCmd::edit().await,
        Cmd::Publish => HandleCmd::publish().await,
        Cmd::Unpublish => HandleCmd::unpublish().await,
        Cmd::Listen { address, port } => {
            HandleCmd::listen(address, port, &connection, run_migrations, static_files_list, handlebars).await
        }
    }
}
