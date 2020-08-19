use chrono::naive::NaiveDateTime;
use async_trait::async_trait;
use crate::models::{ ProjectCategory, ProjectImage, User, model::{ Model, NewModel, UpdatableModel } };
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
    pub user_id: i32,
    pub category: Option<ProjectCategory>,
    pub images: Option<Vec<ProjectImage>>,
    pub user: Option<User>
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

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdatableProject {
    pub id : i32,
    pub title : String,
    pub content : String,
    pub category_id : i32,
    pub user_id : i32
}

#[async_trait]
impl UpdatableModel<Project> for UpdatableProject {
    async fn update (self) -> Result<Project, Error> {
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

#[async_trait]
impl Model<Project> for Project {
    async fn find(project_id: i32) -> Result<Project, Error>
    where Project: 'async_trait{
        let row: Row = Self::db().await.query_one("select * from projects where id = $1 and deleted_at is null;",  &[&project_id]).await?;
        let p = Project::new(&row);
        let p = p.attach_category().await?;
        let p = p.attach_user().await?;
        let p = p.attach_images().await;
        p
    }
    async fn all() -> Result<Vec<Project>, Error>
        where Project: 'async_trait{
        let rows: Vec<Row> = Self::db().await.query("select * from projects where deleted_at is null;", &[]).await?;
        let mut projects = Vec::new();
        for row in rows{
            let p = Project::new(&row);
            let p = p.attach_category().await?;
            let p = p.attach_user().await?;
            let p = p.attach_images().await?;
            projects.push(p);
        }
        Ok(projects)
    }

    async fn update(self) -> Result<Project, Error> {

        let row: Row = Self::db().await.query_one("update projects set category_id = $2, title = $3, slug = $4, content = $5, views_count = $6, likes_count = $7, deleted_at = $8, created_at = $9, updated_at = CURRENT_TIMESTAMP, sketchfab_model_number = $10, user_id = $11 where id = $1 returning *;",
                                    &[&self.id, &self.category_id, &self.title, &self.slug, &self.content, &self.views_count, &self.likes_count, &self.deleted_at, &self.created_at, &self.sketchfab_model_number, &self.user_id]).await?;
        let p = Project::new(&row);
        let p = p.attach_category().await?;
        let p = p.attach_user().await?;
        let p = p.attach_images().await?;
        Ok(p)

    }
    async fn delete(mut self) -> Result<Project, Error>{
        let row = Self::db().await.query_one("update projects set deleted_at = CURRENT_TIMESTAMP where id = $1", &[&self.id]).await?;
        let p = Project::new(&row);
        let p = p.attach_category().await?;
        let p = p.attach_user().await?;
        let p = p.attach_images().await?;
        Ok(p)
    }
}

impl Project{
    pub fn new<'a>(row: &Row) -> Project{
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
            user_id: row.get("user_id"),
            category: None,
            images: None,
            user: None
        }
    }

    pub async fn all_but_not_blog() -> Result<Vec<Project>, Error>{
        let rows: Vec<Row> = Self::db().await.query("select * from projects where deleted_at is null and category_id != 5;", &[]).await?;
        let mut projects = Vec::new();
        for row in rows{
            let p = Project::new(&row);
            let p = p.attach_category().await?;
            let p = p.attach_user().await?;
            let p = p.attach_images().await?;
            projects.push(p);
        }
        Ok(projects)
    }

    pub async fn by_category(cat_id: i32) -> Result<Vec<Project>, Error>{
        let rows: Vec<Row> = Self::db().await.query("select * from projects where category_id = $1;", &[&cat_id]).await?;
        let mut projects = Vec::new();
        for row in rows{
            let project = Project::new(&row);
            let project = project.attach_category().await?;
            let project = project.attach_user().await?;
            let project = project.attach_images().await?;
            projects.push(project);
        }
        Ok(projects)
    }
    pub async fn attach_category(mut self) -> Result<Project, Error>{
        let row = Self::db().await.query_one("select * from project_categories where id = $1", &[&self.category_id]).await?;
        let cat = ProjectCategory::new(&row);
        self.category = Some(cat);
        Ok(self)
    }

    pub async fn attach_images(mut self) -> Result<Project, Error>{
        let rows = Self::db().await.query("select * from project_images where project_id = $1", &[&self.id]).await?;
        let mut images = Vec::new();
        for row in rows{
            let i = ProjectImage::new(&row);
            images.push(i);
        }
        self.images = Some(images);
        Ok(self)
    }

    pub async fn attach_user(mut self) -> Result<Project, Error>{
        let row = Self::db().await.query_one("select * from users where id = $1", &[&self.user_id]).await?;
        let user = User::new(&row);
        let user = user.attach_profile_images().await?;
        self.user = Some(user);
        Ok(self)
    }
}

#[async_trait]
impl NewModel<Project> for NewProject {
    async fn save(self) -> Result<Project, Error>
    where Project: 'async_trait{

        let row: Row = Self::db().await.query_one("insert into projects (category_id, title, slug, content, created_at, sketchfab_model_number, user_id) values ($1, $2, $3, $4, CURRENT_TIMESTAMP, $5, $6) returning *;",
                                    &[&self.category_id, &self.title, &self.slug, &self.content, &self.sketchfab_model_number, &self.user_id]).await?;

        let project = Project::new(&row);
        let project = project.attach_category().await?;
        let project = project.attach_user().await?;
        let project = project.attach_images().await?;
        Ok(project)
    }
}

impl NewProject {
    pub async fn check_slug_unique(self, slug_to_find: String)-> bool {
        let row = Self::db().await.query_one("select id from projects where slug = $1;", &[&slug_to_find]).await;
        row.is_err()
    }
}
