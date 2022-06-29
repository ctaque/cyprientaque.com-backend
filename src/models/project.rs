use crate::command::cli::AppData;
use crate::models::{ProjectCategory, ProjectCategoryHardcoded, ProjectImage, User, NewProjectLike};
use crate::utils::{iso_date_format, utils::Sorter};
use actix_web::{http, web, HttpResponse};
use async_trait::async_trait;
use chrono::naive::NaiveDateTime;
use ipnetwork::IpNetwork;
use postgres::{error::Error, Row};
use rand::distributions::Alphanumeric;
use rand::Rng;
use rest_macro::{
    DeleteInfo, FindInfo, HttpAll, HttpDelete, HttpFind, Model, NewModel, UpdatableModel, HttpAllOptionalQueryParams
};
use rest_macro_derive::{HttpAll, HttpDelete, HttpFind};
use serde::{Deserialize, Serialize};
use serde_json::{json, value::Map};
use slugify::slugify;
use std::env::{temp_dir, var};
use std::fs::{self, File};
use std::io::prelude::*;
use std::process::Command;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Clone, Debug, Serialize, Deserialize, HttpFind, HttpAll, HttpDelete)]
pub struct Project {
    pub id: i32,
    pub category_id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub views_count: i32,
    pub likes_count: i32,
    pub deleted_at: Option<NaiveDateTime>,
    #[serde(with = "iso_date_format")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(with = "iso_date_format")]
    pub updated_at: Option<NaiveDateTime>,
    pub sketchfab_model_number: Option<String>,
    pub user_id: i32,
    pub category: Option<ProjectCategory>,
    pub images: Option<Vec<ProjectImage>>,
    pub user: Option<User>,
    pub is_pro: bool,
    pub bitbucket_project_key: Option<String>,
    pub published: bool,
    pub primary_image: Option<ProjectImage>,
    pub tags: String,
    pub tags_list: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
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

        let params = HttpAllOptionalQueryParams{
            images: Some(true),
            author: Some(true),
            category: Some(true),
            ..Default::default()
        };
        Project::new(&row, &params).await
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

        let params = HttpAllOptionalQueryParams{
            images: Some(true),
            author: Some(true),
            category: Some(true),
            ..Default::default()
        };

        Project::new(&row, &params).await
    }
    async fn all(query: HttpAllOptionalQueryParams) -> Result<Vec<Project>, Error>
    where
        Project: 'async_trait,
    {
        let rows: Vec<Row> = Self::db()
            .await
            .query("select * from projects where deleted_at is null order by id;", &[])
            .await?;
        let mut projects = Vec::new();
        for row in rows {

            let p = Project::new(&row, &query).await;
            if let Ok(project) = p {
                projects.push(project);
            }
        }
        Ok(projects)
    }

    async fn update(self) -> Result<Project, Error> {
        let row: Row = Self::db()
            .await
            .query_one(
                "update projects set
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
                published = $12,
                tags = $13
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
                    &self.tags,
                ],
            )
            .await?;

        let params = HttpAllOptionalQueryParams{
            images: Some(true),
            author: Some(true),
            category: Some(true),
            ..Default::default()
        };

        Project::new(&row, &params).await
    }
    async fn delete(mut self) -> Result<Project, Error> {
        let row = Self::db()
            .await
            .query_one(
                "update projects set deleted_at = CURRENT_TIMESTAMP where id = $1 returning *",
                &[&self.id],
            )
            .await?;

        let params = HttpAllOptionalQueryParams{
            images: Some(true),
            author: Some(true),
            category: Some(true),
            ..Default::default()
        };

        Project::new(&row, &params).await
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
    pub fn new(row: &Row) -> i32 {
        Id { id: row.get("id") }.id
    }
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub s: String,
    pub category_id: i32,
}

#[derive(Deserialize)]
pub struct HttpBlogDetailSlug {
    slug: String,
}

#[derive(Deserialize)]
pub struct BlogIndexQuery{
    tag: Option<String>
}
#[derive(Serialize)]
pub struct TagActive {
    tag: String,
    active: bool,
}


impl Project {
    pub async fn new<'a>(row: &Row, query: &HttpAllOptionalQueryParams) -> Result<Project, Error> {
        let mut p = Project {
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
            primary_image: None,
            tags: row.get("tags"),
            tags_list: Vec::from_iter(row.get::<_, String>("tags").split(",").map(|s| s.trim()).filter(|s| s != &"").map(String::from))
        };

        let attach_cat = query.attach_category();
        let attach_author = query.attach_author();
        let attach_images = query.attach_images();
        let attach_primary_image = query.attach_primary_image();
        if attach_cat {
            p.attach_category().await?;
        };
        if attach_author {
            p.attach_author().await?;
        };
        if attach_images {
            p.attach_images().await?;
        };
        if attach_primary_image {
            let image_res = ProjectImage::get_image_with_max_views_for_project(p.id).await;
            if let Ok(image) = image_res {
                p.primary_image = Some(image);
            }
        };
        Ok(p)
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
        let params = HttpAllOptionalQueryParams {
            category: Some(true),
            author: Some(true),
            images: Some(true),
            ..Default::default()
        };
        for row in rows {
            let p = Project::new(&row, &params).await;
            if let Ok(project) = p {
                projects.push(project);
            };
        }
        Ok(projects)
    }

    pub async fn all_but_not_blog() -> Result<Vec<Project>, Error> {
        let rows: Vec<Row> = Self::db().await.query("select * from projects where deleted_at is null and category_id != 5 and published = true;", &[]).await?;
        let mut projects = Vec::new();
        let params = HttpAllOptionalQueryParams {
            category: Some(true),
            author: Some(true),
            images: Some(true),
            ..Default::default()
        };
        for row in rows {
            let p = Project::new(&row, &params).await;
            if let Ok(project) = p {
                projects.push(project);
            };
        }
        Ok(projects)
    }

    pub async fn by_category(cat_id: i32, query: HttpAllOptionalQueryParams) -> Result<Vec<Project>, Error> {
        let rows: Vec<Row> = Self::db()
            .await
            .query(
                "select * from projects where category_id = $1 and published = true;",
                &[&cat_id],
            )
            .await?;
        let mut projects = Vec::new();
        for row in rows {
            let p = Project::new(&row, &query).await;
            if let Ok(project) = p {
                projects.push(project);
            };
        }
        Ok(projects)
    }

    pub async fn of_category_hardcoded(
        cat: ProjectCategoryHardcoded,
        query: HttpAllOptionalQueryParams
    ) -> Result<Vec<Project>, Error> {
        let rows: Vec<Row> = Self::db()
            .await
            .query(
                "select * from projects where category_id = $1 and published = true and deleted_at is null;",
                &[&cat.value()]
            ).await?;
        let mut projects = Vec::new();
        for row in rows {
            let p = Project::new(&row, &query).await;
            if let Ok(project) = p {
                projects.push(project);
            };
        }
        Ok(projects)
    }

    pub async fn search_projects(terms: String, category_id: i32) -> Result<Vec<i32>, Error> {
        let mut q = String::from("select id from projects where published = true and deleted_at is null and category_id <> 5 and to_tsvector(title || ' ' || content) @@ to_tsquery($1)");
        let mut ids = Vec::new();
        let split: Vec<&str> = terms.split(" ").collect();
        let without_space: Vec<String> = split
            .iter()
            .filter(|v| v.to_string() != String::from(""))
            .map(|v| v.to_string())
            .collect();
        let mut formatted_terms = without_space.join(":* & ").to_string();
        formatted_terms.push_str(":*");
        if category_id != 0 {
            q.push_str(" and category_id  = $2");
            let rows: Vec<Row> = Self::db()
                .await
                .query(q.as_str(), &[&formatted_terms, &category_id])
                .await?;
            for row in rows {
                let id = Id::new(&row);
                ids.push(id);
            }
            Ok(ids)
        } else {
            let rows: Vec<Row> = Self::db()
                .await
                .query(q.as_str(), &[&formatted_terms])
                .await?;
            for row in rows {
                let id = Id::new(&row);
                ids.push(id);
            }
            Ok(ids)
        }
    }

    pub async fn get_uniq_tags(category_id: i32) -> Result<Vec<String>, Error> {
        let rows = Self::db()
            .await
            .query("select tags from projects where tags <> '' and category_id = $1 and published = true", &[&category_id])
            .await?;
        let mut tags: Vec<String> = rows.iter()
            .map::<String, _>(| row | row.get("tags"))
            .collect::<Vec<String>>()
            .join(",")
            .split(',')
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
            .join(",")
            .split(",")
            .map(|current| current.to_string())
            .collect::<Vec<String>>();

        tags.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        Ok(tags)
    }
    pub async fn get_projects_by_tag(tag: String, category_id: i32, query: HttpAllOptionalQueryParams) -> Result<Vec<Project>, Error> {
        let mut q = String::from("select * from projects where published = true and deleted_at is null and tags LIKE CONCAT('%', $1::text, '%')");
        if category_id != 0 {
            q.push_str(" and category_id  = $2");
            let rows: Vec<Row> = Self::db()
                .await
                .query(q.as_str(), &[&tag, &category_id])
                .await?;
            let mut out = Vec::new();
            for row in rows {
                let p = Project::new(&row, &query).await;
                if let Ok(project) = p {
                    out.push(project);
                };
            }
            Ok(out)
        } else {
            let rows: Vec<Row> = Self::db()
                .await
                .query(q.as_str(), &[&tag])
                .await?;
            let mut out = Vec::new();
            for row in rows {
                let p = Project::new(&row, &query).await;
                if let Ok(project) = p {
                    out.push(project);
                };
            }
            Ok(out)
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
        let params = HttpAllOptionalQueryParams{
            images: Some(true),
            author: Some(true),
            category: Some(true),
            ..Default::default()
        };
        let project = Project::new(&row, &params).await?;
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
        let params = HttpAllOptionalQueryParams{
            images: Some(true),
            author: Some(true),
            category: Some(true),
            ..Default::default()
        };
        let project = Project::new(&row, &params).await?;
        Ok(project)
    }

    pub async fn attach_category(&mut self) -> Result<(), Error> {
        let cat:Option<ProjectCategory> = ProjectCategory::find(self.category_id).await.ok();
        self.category = cat;
        Ok(())
    }

    pub async fn attach_images(&mut self) -> Result<(), Error> {
        let rows = Self::db()
            .await
            .query(
                "select * from project_images where project_id = $1 and deleted_at is null",
                &[&self.id],
            )
            .await?;
        let mut images = Vec::new();
        for row in rows {
            let i = ProjectImage::new(&row);
            images.push(i);
        }

        self.primary_image = ProjectImage::get_primary_image(images.clone());
        self.images = Some(images);
        Ok(())
    }

    pub async fn attach_author(&mut self) -> Result<(), Error> {
        let row = Self::db()
            .await
            .query_one("select * from users where id = $1", &[&self.user_id])
            .await?;
        let user = User::new(&row);
        let user = user.attach_profile_images().await?;
        self.user = Some(user);
        Ok(())
    }

    pub async fn get_by_slug(slug: String) -> Option<Project> {
        let result_row = Self::db()
            .await
            .query_one("select * from projects where slug = $1 and published = true and deleted_at is null", &[&slug])
            .await;
        match result_row {
            Ok(row) => {
                let params = HttpAllOptionalQueryParams{
                    images: Some(true),
                    author: Some(true),
                    category: Some(true),
                    ..Default::default()
                };
                let project_res = Project::new(&row, &params).await;
                match project_res {
                    Ok(project) => Some(project),
                    Err(_e) => None
                }
            }
            _ => None,
        }
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
        params: web::Query<HttpAllOptionalQueryParams>
    ) -> Result<HttpResponse, HttpResponse> {
        let result = Project::by_category(info.category_id.into(), params.into_inner()).await;
        match result {
            Ok(res) => Ok(HttpResponse::Ok().body(json!(res))),
            Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
        }
    }

    pub async fn add_view(mut self) -> Result<Project, Error> {
        Self::db()
            .await
            .query_one(
                "UPDATE projects SET views_count = views_count + 1 WHERE id = $1 RETURNING *",
                &[&self.id],
            )
            .await?;
        self.views_count = self.views_count + 1;
        Ok(self)
    }

    pub async fn http_add_view(info: web::Path<Id>) -> Result<HttpResponse, HttpResponse> {
        let result: Result<Project, Error> = Project::find(info.id.into()).await;

        match result {
            Ok(project) => {
                let result = project.add_view().await;
                match result {
                    Ok(updated_project) => Ok(HttpResponse::Ok().body(json!(updated_project))),
                    Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
                }
            }
            Err(err) => Err(HttpResponse::NotFound().body(err.to_string())),
        }
    }

    pub async fn add_like(mut self, ip: IpNetwork) -> Result<Project, Error> {
        let new_project_like = NewProjectLike{
            project_id: self.id,
            ip,
        };

        let opt = new_project_like.toggle().await?;

        if let Some(_) = opt {
            Self::db()
                .await
                .query_one(
                    "UPDATE projects SET likes_count = likes_count + 1 WHERE id = $1 RETURNING *",
                    &[&self.id],
                )
                .await?;
            self.likes_count = self.likes_count + 1;
            Ok(self)
        } else {
            Self::db()
                .await
                .query_one(
                    "UPDATE projects SET likes_count = likes_count - 1 WHERE id = $1 RETURNING *",
                    &[&self.id],
                )
                .await?;
            self.likes_count = self.likes_count - 1;
            Ok(self)
        }

    }

    pub async fn http_add_like(info: web::Path<Id>, req: web::HttpRequest) -> Result<HttpResponse, HttpResponse> {
        let conn_info = req.connection_info();
        let ip: IpNetwork = if let Some(val) = conn_info.realip_remote_addr() {
            let r = IpNetwork::from_str(val);
            if let Ok(curr) = r {
                curr
            } else {
                return Err(HttpResponse::BadRequest().body("Bad ip address"))
            }
        } else {
            return Err(HttpResponse::BadRequest().body("Bad ip address"))
        };
        let result: Result<Project, Error> = Project::find(info.id.into()).await;
        match result {
            Ok(project) => {
                let result = project.add_like(ip).await;
                match result {
                    Ok(updated_project) => Ok(HttpResponse::Ok().body(json!(updated_project))),
                    Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
                }
            }
            Err(err) => Err(HttpResponse::NotFound().body(err.to_string())),
        }
    }

    pub async fn do_i_like(self, ip: IpNetwork) -> Result<bool, Error> {
        let project_like = NewProjectLike{
            project_id: self.id,
            ip,
        };
        Ok(project_like.is_liked().await?)
    }

    pub async fn http_do_i_like(info: web::Path<Id>, req: web::HttpRequest) -> Result<HttpResponse, HttpResponse> {
        let conn_info = req.connection_info();
        let ip: IpNetwork = if let Some(val) = conn_info.realip_remote_addr() {
            let r = IpNetwork::from_str(val);
            if let Ok(curr) = r {
                curr
            } else {
                return Err(HttpResponse::BadRequest().body("Bad ip address"))
            }
        } else {
            return Err(HttpResponse::BadRequest().body("Bad ip address"))
        };

        let result: Result<Project, Error> = Project::find(info.id.into()).await;
        #[derive(Serialize)]
        struct IsLikedResponse {
            value: bool
        }
        match result {
            Ok(project) => {
                let result = project.do_i_like(ip).await;
                match result {
                    Ok(res) => Ok(HttpResponse::Ok().body(json!(IsLikedResponse{ value: res }))),
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

    pub async fn http_blog_detail(
        info: web::Path<HttpBlogDetailSlug>,
        app_data: web::Data<AppData>,
    ) -> Result<HttpResponse, HttpResponse> {
        let opt_article = Self::get_by_slug(info.slug.clone()).await;
        match opt_article {
            None => Err(HttpResponse::Found()
                .header(http::header::LOCATION, "/blog")
                .finish()
                .into_body()),
            Some(article) => {
                let article = article.add_view().await.unwrap();
                let mut data = Map::new();
                data.insert("article".to_string(), json!(article));
                data.insert("base".to_string(), json!("base".to_string()));
                data.insert("title".to_string(), json!(article.title.to_string()));
                let result = app_data.handlebars.render("blog_detail", &data);
                match result {
                    Err(e) => Err(HttpResponse::InternalServerError().body(e.to_string())),
                    Ok(html) => Ok(HttpResponse::Ok().body(html)),
                }
            }
        }
    }

    pub async fn http_blog_index(
        app_data: web::Data<AppData>,
        query: web::Query<BlogIndexQuery>
    ) -> Result<HttpResponse, HttpResponse> {
        let mut default_params: HttpAllOptionalQueryParams = Default::default();
        default_params.primary_image = Some(true);
        let mut default_params_tags: HttpAllOptionalQueryParams = Default::default();
        default_params_tags.primary_image = Some(true);
        let mut articles = match query.tag.clone() {
            Some(tag) =>  Project::get_projects_by_tag(tag, ProjectCategoryHardcoded::Blog.value(), default_params_tags).await.unwrap(),
            None => Self::of_category_hardcoded(ProjectCategoryHardcoded::Blog, default_params).await.unwrap()
        };
        let tags = Self::get_uniq_tags(ProjectCategoryHardcoded::Blog.value()).await.unwrap();
        let tags = tags.into_iter()
        .map::<TagActive, _>(|current| -> TagActive {
            TagActive {
                tag: current.clone(),
                active: if let Some(from_query) = query.tag.clone() {
                    from_query == current
                } else {
                    false
                }
            }
        })
        .collect::<Vec<TagActive>>();
        let mut data = Map::new();

        articles.sort_by(Sorter::CreatedAt.project());
        data.insert("articles".to_string(), json!(articles));
        data.insert("tags".to_string(), json!(tags));
        data.insert("current_tag".to_string(), json!(query.tag.clone()));
        data.insert("base".to_string(), json!("base".to_string()));
        data.insert("title".to_string(), json!("Blog de Cyprien Taque".to_string()));
        let result = app_data.handlebars.render("blog_index", &data);
        match result {
            Err(e) => Err(HttpResponse::InternalServerError().body(e.to_string())),
            Ok(html) => Ok(HttpResponse::Ok().body(html)),
        }
    }

    pub async fn http_text_search_projects(
        info: web::Query<SearchQuery>,
    ) -> Result<HttpResponse, HttpResponse> {
        let result: Result<Vec<i32>, Error> =
            Project::search_projects(info.s.to_string(), info.category_id).await;
        match result {
            Ok(projects) => Ok(HttpResponse::Ok().body(json!(projects))),
            Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
        }
    }

    pub fn pretty_print(self) -> () {
        println!(
            "Project id: {}, Title: {}, published: {}, views_count: {}, likes_count: {}, created_at: {}, updated_at: {}, deleted_at: {}",
            self.id,
            self.title,
            self.published,
            self.views_count,
            self.likes_count,
            self.created_at.and_then(|date| Some(date.to_string())).unwrap_or("null".to_string()),
            self.updated_at.and_then(|date| Some(date.to_string())).unwrap_or("null".to_string()),
            self.deleted_at.and_then(|date| Some(date.to_string())).unwrap_or("null".to_string()),
        );
    }
    pub async fn print_all() -> Result<(), String> {
        let opts: HttpAllOptionalQueryParams = Default::default();
        let result = Project::all(opts).await;
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

        let params = HttpAllOptionalQueryParams{
            images: Some(true),
            author: Some(true),
            category: Some(true),
            ..Default::default()
        };
        Project::new(&row, &params).await
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
