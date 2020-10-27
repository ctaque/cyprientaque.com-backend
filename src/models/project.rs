use crate::models::{ProjectCategory, ProjectImage, User};
use actix_web::{web, HttpResponse};
use async_trait::async_trait;
use chrono::naive::NaiveDateTime;
use postgres::{error::Error, Row};
use rand::distributions::Alphanumeric;
use rand::Rng;
use rest_macro::{
    DeleteInfo, FindInfo, HttpAll, HttpDelete, HttpFind, Model, NewModel, UpdatableModel,
};
use rest_macro_derive::{HttpAll, HttpDelete, HttpFind};
use serde::Deserialize;
use serde_json::json;
use slugify::slugify;
use std::env::{temp_dir, var};
use std::fs::{self, File};
use std::io::prelude::*;
use std::process::Command;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, HttpFind, HttpAll, HttpDelete)]
pub struct Project {
    pub id: i32,
    pub category_id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub views_count: i32,
    pub likes_count: i32,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub sketchfab_model_number: Option<String>,
    pub user_id: i32,
    pub category: Option<ProjectCategory>,
    pub images: Option<Vec<ProjectImage>>,
    pub user: Option<User>,
    pub is_pro: bool,
    pub bitbucket_project_key: Option<String>,
    pub published: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct NewProject {
    pub category_id: i32,
    pub title: String,
    pub slug: Option<String>,
    pub content: String,
    pub sketchfab_model_number: Option<String>,
    pub user_id: i32,
    pub is_pro: bool,
    pub bitbucket_project_key: Option<String>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdatableProject {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub category_id: i32,
    pub user_id: i32,
}

#[async_trait]
impl UpdatableModel<Project> for UpdatableProject {
    async fn update(self) -> Result<Project, Error> {
        let row = Self::db().await.query_one(
            "update projects set category_id = $1, title = $2, content = $3, user_id = $4 where id = $5 returning *",
            &[&self.category_id, &self.title, &self.content, &self.user_id, &self.id]
        ).await?;
        let project = Project::new(&row);
        let project = project.attach_category().await?;
        let project = project.attach_user().await?;
        let project = project.attach_images().await?;
        Ok(project)
    }
}

impl UpdatableProject {
    pub async fn http_update(
        info: web::Json<UpdatableProject>,
    ) -> Result<HttpResponse, HttpResponse> {
        let from_db: Result<Project, Error> = Project::find(info.id.into()).await;

        match from_db {
            Ok(_) => {
                let result = info.into_inner().update().await;
                match result {
                    Ok(updated_project) => Ok(HttpResponse::Ok().body(json!(updated_project))),
                    Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
                }
            }
            Err(err) => Err(HttpResponse::NotFound().body(err.to_string())),
        }
    }
}

#[async_trait]
impl Model<Project> for Project {
    async fn find(project_id: i32) -> Result<Project, Error>
    where
        Project: 'async_trait,
    {
        let row: Row = Self::db()
            .await
            .query_one(
                "select * from projects where id = $1 and deleted_at is null;",
                &[&project_id],
            )
            .await?;
        let p = Project::new(&row);
        let p = p.attach_category().await?;
        let p = p.attach_user().await?;
        let p = p.attach_images().await;
        p
    }
    async fn all() -> Result<Vec<Project>, Error>
    where
        Project: 'async_trait,
    {
        let rows: Vec<Row> = Self::db()
            .await
            .query("select * from projects where deleted_at is null;", &[])
            .await?;
        let mut projects = Vec::new();
        for row in rows {
            let p = Project::new(&row);
            let p = p.attach_category().await?;
            let p = p.attach_user().await?;
            let p = p.attach_images().await?;
            projects.push(p);
        }
        Ok(projects)
    }

    async fn update(self) -> Result<Project, Error> {
        let row: Row = Self::db()
            .await
            .query_one(
                "update projects set (
            category_id = $2,
            title = $3,
            slug = $4,
            content = $5,
            views_count = $6,
            likes_count = $7,
            deleted_at = $8,
            created_at = $9,
            updated_at = CURRENT_TIMESTAMP,
            sketchfab_model_number = $10,
            user_id = $11,
            published = $12
            )
            where id = $1 returning *;",
                &[
                    &self.id,
                    &self.category_id,
                    &self.title,
                    &self.slug,
                    &self.content,
                    &self.views_count,
                    &self.likes_count,
                    &self.deleted_at,
                    &self.created_at,
                    &self.sketchfab_model_number,
                    &self.user_id,
                    &self.published,
                ],
            )
            .await?;
        let p = Project::new(&row);
        let p = p.attach_category().await?;
        let p = p.attach_user().await?;
        let p = p.attach_images().await?;
        Ok(p)
    }
    async fn delete(mut self) -> Result<Project, Error> {
        let row = Self::db()
            .await
            .query_one(
                "update projects set deleted_at = CURRENT_TIMESTAMP where id = $1 returning *",
                &[&self.id],
            )
            .await?;
        let p = Project::new(&row);
        let p = p.attach_category().await?;
        let p = p.attach_user().await?;
        let p = p.attach_images().await?;
        Ok(p)
    }
}

#[derive(Deserialize)]
pub struct CategoryId {
    pub category_id: i32,
}

#[derive(Deserialize)]
pub struct Id {
    pub id: i32,
}

impl Id {
    pub fn new(row: &Row) -> Id {
        Id {
            id: row.get("id")
        }
    }
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub s: String,
    pub category_id: i32,
}

impl Project {
    pub fn new<'a>(row: &Row) -> Project {
        Project {
            id: row.get("id"),
            category_id: row.get("category_id"),
            title: row.get("title"),
            slug: row.get("slug"),
            content: row.get("content"),
            views_count: row.get("views_count"),
            likes_count: row.get("likes_count"),
            deleted_at: row.get("deleted_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            sketchfab_model_number: row.get("sketchfab_model_number"),
            bitbucket_project_key: row.get("bitbucket_project_key"),
            user_id: row.get("user_id"),
            is_pro: row.get("is_pro"),
            published: row.get("published"),
            category: None,
            images: None,
            user: None,
        }
    }

    pub async fn all_published() -> Result<Vec<Project>, Error> {
        let rows: Vec<Row> = Self::db()
            .await
            .query(
                "select * from projects where deleted_at is null and published = true;",
                &[],
            )
            .await?;
        let mut projects = Vec::new();
        for row in rows {
            let p = Project::new(&row);
            let p = p.attach_category().await?;
            let p = p.attach_user().await?;
            let p = p.attach_images().await?;
            projects.push(p);
        }
        Ok(projects)
    }

    pub async fn all_but_not_blog() -> Result<Vec<Project>, Error> {
        let rows: Vec<Row> = Self::db().await.query("select * from projects where deleted_at is null and category_id != 5 and published = true;", &[]).await?;
        let mut projects = Vec::new();
        for row in rows {
            let p = Project::new(&row);
            let p = p.attach_category().await?;
            let p = p.attach_user().await?;
            let p = p.attach_images().await?;
            projects.push(p);
        }
        Ok(projects)
    }

    pub async fn by_category(cat_id: i32) -> Result<Vec<Project>, Error> {
        let rows: Vec<Row> = Self::db()
            .await
            .query(
                "select * from projects where category_id = $1 and published = true;",
                &[&cat_id],
            )
            .await?;
        let mut projects = Vec::new();
        for row in rows {
            let project = Project::new(&row);
            let project = project.attach_category().await?;
            let project = project.attach_user().await?;
            let project = project.attach_images().await?;
            projects.push(project);
        }
        Ok(projects)
    }

    pub async fn search_projects(terms: String, category_id: i32) -> Result<Vec<i32>, Error> {
        let mut q = String::from("select id from projects where published = true and deleted_at is null and category_id <> 5 and to_tsvector(title || ' ' || content) @@ to_tsquery($1)");
        let mut ids = Vec::new();
        let split: Vec<&str> = terms.split(" ").collect();
        let without_space: Vec<String> = split.iter().filter(|v| v.to_string() != String::from("")).map(|v| v.to_string()).collect();
        let mut formatted_terms = without_space.join(":* & ").to_string();
        formatted_terms.push_str(":*");
        if category_id != 0 {
            q.push_str(" and category_id  = $2");
            let rows: Vec<Row> = Self::db().await.query(q.as_str(), &[&formatted_terms, &category_id]).await?;
            for row in rows {
                let id = Id::new(&row);
                ids.push(id);
            }
            Ok(ids.iter().map(|id| id.id).collect())
        } else {
            let rows: Vec<Row> = Self::db().await.query(q.as_str(), &[&formatted_terms]).await?;
            for row in rows {
                let id = Id::new(&row);
                ids.push(id);
            }
            Ok(ids.iter().map(|id| id.id).collect())
        }
    }

    pub async fn publish(self) -> Result<Project, Error> {
        let row = Self::db()
            .await
            .query_one(
                "UPDATE projects set published = true where id = $1 returning *;",
                &[&self.id],
            )
            .await?;
        let project = Project::new(&row);
        let project = project.attach_images().await?;
        let project = project.attach_user().await?;
        let project = project.attach_category().await?;
        Ok(project)
    }

    pub async fn unpublish(self) -> Result<Project, Error> {
        let row = Self::db()
            .await
            .query_one(
                "UPDATE projects set published = false where id = $1 returning *;",
                &[&self.id],
            )
            .await?;
        let project = Project::new(&row);
        let project = project.attach_images().await?;
        let project = project.attach_user().await?;
        let project = project.attach_category().await?;
        Ok(project)
    }

    pub async fn attach_category(mut self) -> Result<Project, Error> {
        let row = Self::db()
            .await
            .query_one(
                "select * from project_categories where id = $1",
                &[&self.category_id],
            )
            .await?;
        let cat = ProjectCategory::new(&row);
        self.category = Some(cat);
        Ok(self)
    }

    pub async fn attach_images(mut self) -> Result<Project, Error> {
        let rows = Self::db()
            .await
            .query(
                "select * from project_images where project_id = $1",
                &[&self.id],
            )
            .await?;
        let mut images = Vec::new();
        for row in rows {
            let i = ProjectImage::new(&row);
            images.push(i);
        }
        self.images = Some(images);
        Ok(self)
    }

    pub async fn attach_user(mut self) -> Result<Project, Error> {
        let row = Self::db()
            .await
            .query_one("select * from users where id = $1", &[&self.user_id])
            .await?;
        let user = User::new(&row);
        let user = user.attach_profile_images().await?;
        self.user = Some(user);
        Ok(self)
    }

    pub async fn http_get_published_projects() -> Result<HttpResponse, HttpResponse> {
        let result = Project::all_published().await;
        match result {
            Ok(res) => Ok(HttpResponse::Ok().body(json!(res))),
            Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
        }
    }

    pub async fn http_get_projects_but_not_blog() -> Result<HttpResponse, HttpResponse> {
        let result = Project::all_but_not_blog().await;
        match result {
            Ok(res) => Ok(HttpResponse::Ok().body(json!(res))),
            Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
        }
    }

    pub async fn http_get_projects_by_category(
        info: web::Path<CategoryId>,
    ) -> Result<HttpResponse, HttpResponse> {
        let result = Project::by_category(info.category_id.into()).await;
        match result {
            Ok(res) => Ok(HttpResponse::Ok().body(json!(res))),
            Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
        }
    }

    pub async fn http_add_view(info: web::Path<Id>) -> Result<HttpResponse, HttpResponse> {
        let result: Result<Project, Error> = Project::find(info.id.into()).await;

        match result {
            Ok(mut project) => {
                project.views_count = project.views_count + 1;
                let result = project.update().await;
                match result {
                    Ok(updated_project) => Ok(HttpResponse::Ok().body(json!(updated_project))),
                    Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
                }
            }
            Err(err) => Err(HttpResponse::NotFound().body(err.to_string())),
        }
    }

    pub async fn http_add_like(info: web::Path<Id>) -> Result<HttpResponse, HttpResponse> {
        let result: Result<Project, Error> = Project::find(info.id.into()).await;
        match result {
            Ok(mut project) => {
                project.likes_count = project.likes_count + 1;
                let result = project.update().await;
                match result {
                    Ok(updated_project) => Ok(HttpResponse::Ok().body(json!(updated_project))),
                    Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
                }
            }
            Err(err) => Err(HttpResponse::NotFound().body(err.to_string())),
        }
    }

    pub async fn http_publish_project(info: web::Path<Id>) -> Result<HttpResponse, HttpResponse> {
        let result: Result<Project, Error> = Project::find(info.id.into()).await;
        match result {
            Ok(project) => {
                let result = project.publish().await;
                match result {
                    Ok(published) => Ok(HttpResponse::Ok().body(json!(published))),
                    Err(err) => {
                        println!("{}", err.to_string());
                        Err(HttpResponse::InternalServerError().body(err.to_string()))
                    }
                }
            }
            Err(err) => Err(HttpResponse::NotFound().body(err.to_string())),
        }
    }

    pub async fn http_unpublish_project(info: web::Path<Id>) -> Result<HttpResponse, HttpResponse> {
        let result: Result<Project, Error> = Project::find(info.id.into()).await;
        match result {
            Ok(project) => {
                let result = project.unpublish().await;
                match result {
                    Ok(published) => Ok(HttpResponse::Ok().body(json!(published))),
                    Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
                }
            }
            Err(err) => Err(HttpResponse::NotFound().body(err.to_string())),
        }
    }

    pub async fn http_text_search_projects(
        info: web::Query<SearchQuery>,
    ) -> Result<HttpResponse, HttpResponse> {
        let result: Result<Vec<i32>, Error> = Project::search_projects(info.s.to_string(), info.category_id).await;
        match result {
            Ok(projects) => Ok(HttpResponse::Ok().body(json!(projects))),
            Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
        }
    }

    pub fn pretty_print(self) -> () {
        println!(
            "Project id: {}, Title: {}, published: {}, created_at: {}, updated_at: {}, deleted_at: {}",
            self.id,
            self.title,
            self.published,
            self.created_at.and_then(|date| Some(date.to_string())).unwrap_or("null".to_string()),
            self.updated_at.and_then(|date| Some(date.to_string())).unwrap_or("null".to_string()),
            self.deleted_at.and_then(|date| Some(date.to_string())).unwrap_or("null".to_string()),
        );
    }
    pub async fn print_all() -> Result<(), String> {
        let result = Project::all().await;
        match result {
            Ok(projects) => {
                for project in projects {
                    project.pretty_print();
                }
                Ok(())
            }
            Err(err) => Err(err.to_string()),
        }
    }
    pub fn selectify(projects: &Vec<Project>) -> Vec<String> {
        projects
            .into_iter()
            .map(|x| format!("id: {}, title: {}", x.id.to_string(), x.title))
            .collect()
    }

    fn gen_random_string() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .collect::<String>()
    }

    fn gen_tmp_filename() -> std::path::PathBuf {
        let name = Self::gen_random_string();
        let mut path = temp_dir();
        path.push(format!("{}.md", name));
        path
    }

    pub fn cli_edit(&mut self) -> Project {
        let editor = var("EDITOR");
        let editor = match editor {
            Ok(editor) => editor,
            _ => "vim".to_string(),
        };
        let file_name = Self::gen_tmp_filename();
        let mut file = File::create(&file_name).unwrap();
        let mut w = Vec::new();
        write!(&mut w, "{}", &self.content).unwrap();
        file.write(&w).unwrap();
        Command::new(editor)
            .arg(&file_name)
            .status()
            .expect("Cannot open file");
        let contents =
            fs::read_to_string(&file_name).expect("Something went wrong reading the file");
        self.content = contents;
        self.to_owned()
    }
}

#[async_trait]
impl NewModel<Project> for NewProject {
    async fn save(self) -> Result<Project, Error>
    where
        Project: 'async_trait,
    {
        let row: Row = Self::db().await.query_one("insert into projects (category_id, title, slug, content, created_at, sketchfab_model_number, user_id, is_pro, bitbucket_project_key) values ($1, $2, $3, $4, CURRENT_TIMESTAMP, $5, $6, $7, $8) returning *;",
                                    &[&self.category_id, &self.title, &self.slug, &self.content, &self.sketchfab_model_number, &self.user_id, &self.is_pro, &self.bitbucket_project_key]).await?;

        let project = Project::new(&row);
        let project = project.attach_category().await?;
        let project = project.attach_user().await?;
        let project = project.attach_images().await?;
        Ok(project)
    }
}

impl NewProject {
    pub async fn check_slug_unique(slug_to_find: String) -> bool {
        let row = Self::db()
            .await
            .query_one("select id from projects where slug = $1;", &[&slug_to_find])
            .await;
        row.is_err()
    }

    pub async fn http_create(
        mut new_project: web::Json<NewProject>,
    ) -> Result<HttpResponse, HttpResponse> {
        let slug: String = slugify!(&new_project.title);
        let is_unique = Self::check_slug_unique(slug.clone()).await;
        if !is_unique {
            Err(HttpResponse::BadRequest().body("Slug already used"))
        } else {
            new_project.slug = Some(slug);
            let result = new_project.clone().save().await;
            match result {
                Ok(project) => Ok(HttpResponse::Ok().body(json!(project))),
                Err(err) => Err(HttpResponse::BadRequest().body(err.to_string())),
            }
        }
    }

    fn gen_random_string() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .collect::<String>()
    }

    fn gen_tmp_filename() -> std::path::PathBuf {
        let name = Self::gen_random_string();
        let mut path = temp_dir();
        path.push(format!("{}.md", name));
        path
    }

    pub fn cli_edit(&mut self) -> &mut NewProject {
        let editor = var("EDITOR");
        let editor = match editor {
            Ok(editor) => editor,
            _ => "vim".to_string(),
        };
        let file_name = Self::gen_tmp_filename();
        let mut file = File::create(&file_name).unwrap();
        let mut w = Vec::new();
        write!(&mut w, "{}", &self.content).unwrap();
        file.write(&w).unwrap();
        Command::new(editor)
            .arg(&file_name)
            .status()
            .expect("Cannot open file");
        let contents =
            fs::read_to_string(&file_name).expect("Something went wrong reading the file");
        self.content = contents;
        self
    }
}
