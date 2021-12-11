use crate::middleware::auth_middleware;
use crate::models::{
    Bitbucket, NewProject, NewProjectImage, Project, ProjectCategory, ProjectImage,
    ProjectImageCategory, UpdatableProject,
};
use actix_cors::Cors;
use actix_web::{
    http,
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use clap::arg_enum;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use diesel::pg::PgConnection;
use diesel_migrations::RunMigrationsError;
use env_logger;
use handlebars::Handlebars;
use rest_macro::{HttpAll, HttpAllOptionalQueryParams, HttpDelete, HttpFind};
use rest_macro::{Model, NewModel};
use slugify::slugify;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;
use url::Url;

arg_enum! {
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
pub enum Cmd {
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
    #[structopt(name = "publish")]
    Publish,
    #[structopt(name = "unpublish")]
    Unpublish,
    #[structopt(name = "edit")]
    Edit,
    #[structopt(name = "add-image")]
    AddImage {
        #[structopt(short = "f", long = "file")]
        file: String,
    },
    #[structopt(name = "change-title")]
    ChangeTitle,
    #[structopt(name = "edit-tags")]
    EditTags,
}

pub struct HandleCmd;

type RunMigrationCb = fn(connection: &PgConnection) -> Result<(), RunMigrationsError>;

type StaticFiles = fn(path: web::Path<String>) -> HttpResponse;

pub struct AppData {
    pub handlebars: Handlebars<'static>,
}

impl HandleCmd {
    pub async fn list() -> std::io::Result<()> {
        let entities = vec![
            Entity::projects,
            Entity::categories,
            Entity::image_categories,
        ];
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
            Entity::image_categories => ProjectImageCategory::print_all().await,
        };
        res.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }

    pub async fn create() -> std::io::Result<()> {
        let options: HttpAllOptionalQueryParams = Default::default();
        let categories: Vec<ProjectCategory> = ProjectCategory::all(options).await.unwrap();
        let selectified_categories: Vec<String> =
            ProjectCategory::selectify_categories(&categories);
        loop {
            let title = Input::<String>::new().with_prompt("Titre").interact();
            let title: String = match title {
                Ok(title) => title,
                Err(_) => continue,
            };
            let slug: String = slugify!(&title);
            let is_unique = NewProject::check_slug_unique(slug.clone()).await;
            if !is_unique {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Slug already used".to_string(),
                ));
            }
            loop {
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Choisir une catégorie")
                    .default(0)
                    .items(&selectified_categories[..])
                    .interact()
                    .unwrap();
                let selected_category: &ProjectCategory = categories.get(selection).unwrap();
                let is_pro = Confirm::new()
                    .with_prompt("Le projet est il un projet professionnel ?")
                    .interact()
                    .unwrap();
                let has_bitbucket_project_key = Confirm::new()
                    .with_prompt("Le projet a-t'il une clé de projet bitbucket ?")
                    .interact()
                    .unwrap();
                let bitbucket_project_key = match has_bitbucket_project_key {
                    true => Input::<String>::new()
                        .with_prompt("Quelle est la clé du projet Bitbucket ?")
                        .interact()
                        .ok(),
                    false => Option::None,
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
                    }
                    Err(err) => println!("{}", err),
                }
                break;
            }
            break;
        }
        Ok(())
    }

    pub async fn add_image(file: String) -> std::io::Result<()> {
        let options_projects: HttpAllOptionalQueryParams = Default::default();
        let options_images_categories: HttpAllOptionalQueryParams = Default::default();
        let projects: Vec<Project> = Project::all(options_projects).await.unwrap();
        let selectified_projects: Vec<String> = Project::selectify(&projects);
        let categories: Vec<ProjectImageCategory> = ProjectImageCategory::all(options_images_categories).await.unwrap();
        let selectified_categories: Vec<String> = ProjectImageCategory::selectify(&categories);
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choisir un projet")
            .default(0)
            .paged(true)
            .items(&selectified_projects[..])
            .interact()
            .unwrap();
        let selected_project: &Project = projects.get(selection).unwrap();

        let image_category_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choisir une catégorie")
            .default(0)
            .paged(true)
            .items(&selectified_categories[..])
            .interact()
            .unwrap();
        let selected_category = categories.get(image_category_selection).unwrap();
        let is_cover_image: bool = Confirm::new()
            .with_prompt("Is it the cover image?")
            .interact()
            .unwrap();

        let parsed = Url::parse(&file);
        match parsed {
            Ok(url) => {
                let path = url.path();
                let split_path = path.clone().split("/");
                let file_name: String = split_path
                    .collect::<Vec<&str>>()
                    .pop()
                    .expect("Cannot read file name from url path")
                    .to_string();
                let client = reqwest::Client::new();
                let resp = client.get(&url.to_string());
                let i = NewProjectImage::new(
                    is_cover_image,
                    selected_project.id,
                    selected_category.id,
                    file_name,
                );
                if let Ok(res) = resp.send().await {
                    let image_data: Vec<u8> = res
                        .bytes()
                        .await
                        .expect("Invalid Image downloaded")
                        .to_vec();
                    let image_350_data = NewProjectImage::generate_size(350.0, image_data.clone())
                        .expect("Failed generating size 350px");
                    let image_1500_data =
                        NewProjectImage::generate_size(1500.0, image_data.clone())
                            .expect("Failed generating size 1500px");
                    NewProjectImage::upload_to_s3(&i.w350_keyname, image_350_data.to_vec())
                        .await
                        .expect("Failed uploading w350 image");
                    println!("Uploaded image 350px to s3");
                    NewProjectImage::upload_to_s3(&i.w1500_keyname, image_1500_data.to_vec())
                        .await
                        .expect("Failed uploading w1500 image");
                    println!("Uploaded image 1500px to s3");
                    NewProjectImage::upload_to_s3(&i.original_keyname, image_data)
                        .await
                        .expect("Failed uploading original size");
                    println!("Uploaded original image to s3");
                    return match i.save().await {
                        Ok(_image) => {
                            println!("Successfully saved image");
                            Ok(())
                        }
                        Err(err) => Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Failed Saving image, error: {}", err.to_string()),
                        )),
                    };
                } else {
                    println!("Cannot GET image");
                }
            }
            _ => {
                let split_path = file.split("/");
                let file_name: String = split_path
                    .collect::<Vec<&str>>()
                    .pop()
                    .expect("Cannot read file name from file path")
                    .to_string();
                let mut file = File::open(file).expect("Failed to open image");
                let mut image_data = Vec::new();
                file.read_to_end(&mut image_data)
                    .expect("Failed to read image");
                let i = NewProjectImage::new(
                    is_cover_image,
                    selected_project.id,
                    selected_category.id,
                    file_name,
                );
                let image_350_data = NewProjectImage::generate_size(350.0, image_data.clone())
                    .expect("Failed generating size 350px");
                let image_1500_data = NewProjectImage::generate_size(1500.0, image_data.clone())
                    .expect("Failed generating size 1500px");
                NewProjectImage::upload_to_s3(&i.w350_keyname, image_350_data.to_vec())
                    .await
                    .expect("Failed uploading w350 image");
                println!("Uploaded image 350px to s3");
                NewProjectImage::upload_to_s3(&i.w1500_keyname, image_1500_data.to_vec())
                    .await
                    .expect("Failed uploading w1500 image");
                println!("Uploaded image 1500px to s3");
                NewProjectImage::upload_to_s3(&i.original_keyname, image_data)
                    .await
                    .expect("Failed uploading original size");
                println!("Uploaded original image to s3");
                return match i.save().await {
                    Ok(_image) => {
                        println!("Successfully saved image");
                        Ok(())
                    }
                    Err(err) => Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed Saving image, error: {}", err.to_string()),
                    )),
                };
            }
        }
        Ok(())
    }

    pub async fn edit() -> std::io::Result<()> {
        let options: HttpAllOptionalQueryParams = Default::default();
        let projects: Vec<Project> = Project::all(options).await.unwrap();
        let selectified_projects: Vec<String> = Project::selectify(&projects);
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choisir un projet")
            .default(0)
            .paged(true)
            .items(&selectified_projects[..])
            .interact()
            .unwrap();
        let mut selected_project: Project = projects.get(selection).unwrap().to_owned();
        let project_to_save = selected_project.cli_edit();
        let out = project_to_save.update().await;
        match out {
            Ok(project_algolia) => {
                println!("{}", "Success !");
                project_algolia.pretty_print()
            }
            Err(err) => println!("{}", err),
        }
        Ok(())
    }

    pub async fn change_title() -> std::io::Result<()> {
        let options: HttpAllOptionalQueryParams = Default::default();
        let projects: Vec<Project> = Project::all(options).await.unwrap();
        let selectified_projects: Vec<String> = Project::selectify(&projects);
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choisir un projet")
            .default(0)
            .paged(true)
            .items(&selectified_projects[..])
            .interact()
            .unwrap();
        let mut selected_project: Project = projects.get(selection).unwrap().to_owned();

        let new_title = Input::<String>::new()
                         .with_prompt("New title?")
                         .with_initial_text(&selected_project.title)
                         .interact().unwrap();
        let slug: String = slugify!(&new_title);
        let found = Project::get_by_slug(slug.clone()).await;
        match found {
            Some(p) => {
                if &p.id != &selected_project.id {
                    return Err(
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            String::from("Slug already used")
                        )
                    )
                } else {
                    println!("Slug unchanged");
                    return Ok(())
                }
            }
            None =>  {
                selected_project.title = new_title;
                selected_project.slug = slug;
                selected_project.update().await.expect("Couldn't save project :/");
                println!("Project saved !");
                return Ok(())
            }
        }
    }

    pub async fn edit_tags() -> std::io::Result<()> {
        let options: HttpAllOptionalQueryParams = Default::default();
        let projects: Vec<Project> = Project::all(options).await.unwrap();
        let selectified_projects: Vec<String> = Project::selectify(&projects);
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choisir un projet")
            .default(0)
            .paged(true)
            .items(&selectified_projects[..])
            .interact()
            .unwrap();
        let mut selected_project: Project = projects.get(selection).unwrap().to_owned();

        let new_tags = Input::<String>::new()
                         .with_prompt("New tags?")
                         .with_initial_text(&selected_project.tags)
                         .interact().unwrap();
        selected_project.tags = new_tags;
        selected_project.update().await.expect("Couldn't save project :/");
        return Ok(())
    }
    pub async fn publish() -> std::io::Result<()> {
        let options: HttpAllOptionalQueryParams = Default::default();
        let projects: Vec<Project> = Project::all(options).await.unwrap();
        let selectified_projects: Vec<String> = Project::selectify(&projects);
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choisir un projet")
            .default(0)
            .paged(true)
            .items(&selectified_projects[..])
            .interact()
            .unwrap();
        let selected_project: Project = projects.get(selection).unwrap().to_owned();
        let result = selected_project.publish().await;
        match result {
            Ok(project) => println!("Successfully published project: \"{}\"", project.title),
            Err(err) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error while publishing: {}", err.to_string()),
                ))
            }
        }
        Ok(())
    }

    pub async fn unpublish() -> std::io::Result<()> {
        let options: HttpAllOptionalQueryParams = Default::default();
        let projects: Vec<Project> = Project::all(options).await.unwrap();
        let selectified_projects: Vec<String> = Project::selectify(&projects);
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choisir un projet")
            .default(0)
            .paged(true)
            .items(&selectified_projects[..])
            .interact()
            .unwrap();
        let selected_project: Project = projects.get(selection).unwrap().to_owned();
        let result = selected_project.unpublish().await;
        match result {
            Ok(project) => println!("Successfully unpublished project {}", project.title),
            Err(err) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error while unpublishing: {}", err.to_string()),
                ))
            }
        }
        Ok(())
    }

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

    pub async fn listen(
        address: String,
        port: String,
        connection: &PgConnection,
        run_migrations: RunMigrationCb,
        hb: Handlebars<'static>,
        static_files: StaticFiles,
    ) -> std::io::Result<()> {
        let migration_run = run_migrations(&connection);
        match migration_run {
            Err(e) => match e {
                RunMigrationsError::MigrationError(e) => {
                    panic!("Error while migrating : {}", e.to_string())
                }
                RunMigrationsError::QueryError(e) => {
                    panic!("Error while migrating : {}", e.to_string())
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

        let app_data = Data::new(AppData { handlebars: hb });

        let server = HttpServer::new(move || {
            App::new()
                .wrap(auth_middleware::Authentication)
                .wrap(Logger::default())
                .wrap(
                    Cors::default() // <- Construct CORS middleware builder
                        .allowed_origin(match is_prod {
                            true => "https://www.cyprientaque.com",
                            false => "http://localhost:3000",
                        })
                        .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
                        .allowed_headers(vec![
                            http::header::AUTHORIZATION,
                            http::header::ACCEPT,
                            http::header::CONTENT_TYPE,
                        ])
                        .max_age(3600)
                )
                .service(web::resource("/static/{_:.*}", ).route(web::get().to(static_files)))
                .app_data(web::PayloadConfig::new(900000000000000000))
                .app_data(app_data.clone())
                .route("/blog/{slug}", web::get().to(Project::http_blog_detail))
                .route("/blog", web::get().to(Project::http_blog_index))
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
                    "/projects/search",
                    web::get().to(Project::http_text_search_projects),
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
                .route(
                    "/bitbucket/accessToken",
                    web::get().to(Bitbucket::access_token),
                )
                .route(
                    "/bitbucket/refreshToken",
                    web::get().to(Bitbucket::refresh_token),
                )
                .route("/", web::get().to(Self::index))
                .route("*", web::get().to(Self::not_found_redirect))
        })
        .bind(&addr)?
        .run()
        .await?;
        sys.await?;
        Ok(server)
    }
}
