use chrono::naive::NaiveDateTime;
use async_trait::async_trait;
use crate::models::{ model::{ Model, NewModel } };
use postgres::{ Row, error::Error };


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
    pub user_id: i32
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct NewProject {
    pub category_id: i32,
    pub title: String,
    pub slug: Option<String>,
    pub content: String,
    pub sketchfab_model_number: Option<String>,
    pub user_id: i32
}

#[async_trait]
impl Model<Project> for Project {
    async fn find(project_id: i32) -> Result<Project, Error>
    where Project: 'async_trait{
        let row: Row = Self::db().await.query_one("select * from projects where id = $1 and deleted_at = null;",  &[&project_id]).await?;
        Ok(Project::new(row))
    }
    async fn all() -> Result<Vec<Project>, Error>
        where Project: 'async_trait{
        let rows: Vec<Row> = Self::db().await.query("select * from projects;", &[]).await?;
        let mut projects = Vec::new();
        for row in rows{
            projects.push(Project::new(row));
        }
        Ok(projects)
    }
    async fn update(self) -> Result<Project, Error> {

        let row: Row = Self::db().await.query_one("update projects set category_id = $2, title = $3, slug = $4, content = $5, views_count = $6, likes_count = $7, deleted_at = $8, created_at = $9, updated_at = CURRENT_TIMESTAMP, sketchfab_model_number = $10, user_id = $11 where id = $1 returning *;",
                                    &[&self.id, &self.category_id, &self.title, &self.slug, &self.content, &self.views_count, &self.likes_count, &self.deleted_at, &self.created_at, &self.sketchfab_model_number, &self.user_id]).await?;
        Ok(Project::new(row))

    }
    async fn delete(mut self) -> Result<Project, Error>{
        self.deleted_at = Option::Some(chrono::offset::Local::now().naive_local());
        let row = Self::db().await.query_one("update projects set deleted_at = $2 where id = $1", &[&self.id, &self.deleted_at]).await?;
        Ok(Project::new(row))
    }
}

impl Project{
    pub fn new(row: Row) -> Project{
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
            user_id: row.get("user_id")
        }
    }
    pub async fn by_category(cat_id: i32) -> Result<Vec<Project>, Error>{
        let rows: Vec<Row> = Self::db().await.query("select * from projects where category_id = $1;", &[&cat_id]).await?;
        let mut projects = Vec::new();
        for row in rows{
            projects.push(Project::new(row));
        }
        Ok(projects)
    }
}

#[async_trait]
impl NewModel<Project> for NewProject {
    async fn save(self) -> Result<Project, Error>
    where Project: 'async_trait{

        let row: Row = Self::db().await.query_one("insert into projects (category_id, title, slug, content, created_at, sketchfab_model_number, user_id) values ($1, $2, $3, $4, CURRENT_TIMESTAMP, $6, $7);",
                                    &[&self.category_id, &self.title, &self.slug, &self.content, &self.sketchfab_model_number, &self.user_id]).await?;
        Ok(Project::new(row))
    }
}

impl NewProject {
    pub async fn check_slug_unique(self, slug_to_find: String)-> bool {
        let row = Self::db().await.query_one("select id from projects where slug = $1;", &[&slug_to_find]).await;
        row.is_err()
    }
}
