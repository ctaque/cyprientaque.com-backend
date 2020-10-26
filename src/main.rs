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
use diesel::pg::PgConnection;
use diesel_migrations::{embed_migrations, RunMigrationsError};
use dotenv::dotenv;
use structopt::StructOpt;
use tokio;

embed_migrations!();

fn run_migrations(connection: &PgConnection) -> Result<(), RunMigrationsError> {
    embedded_migrations::run(connection)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let connection = establish_connection();
    let args = Cmd::from_args();
    match args {
        Cmd::List => HandleCmd::list().await,
        Cmd::Create => HandleCmd::create().await,
        Cmd::AddImage { file } => HandleCmd::add_image(file).await,
        Cmd::Edit => HandleCmd::edit().await,
        Cmd::Publish => HandleCmd::publish().await,
        Cmd::Unpublish => HandleCmd::unpublish().await,
        Cmd::Listen { address, port } => HandleCmd::listen(address, port, &connection, run_migrations).await
    }
}
