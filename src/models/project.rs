use chrono::naive::NaiveDateTime;
use diesel::result::Error;
use crate::models::model::{ Model, NewModel };
use diesel::pg::PgConnection;
use super::super::diesel::prelude::*;
use crate::schema::projects;


#[derive(Clone, Debug, Queryable, Identifiable, AsChangeset, serde::Serialize, serde::Deserialize)]
#[table_name="projects"]
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

#[derive(Clone, Insertable, serde::Serialize, serde::Deserialize)]
#[table_name="projects"]
pub struct NewProject {
    pub category_id: i32,
    pub title: String,
    pub slug: Option<String>,
    pub content: String,
    pub sketchfab_model_number: Option<String>,
    pub user_id: i32
}

impl Model<Project> for Project {
    fn find(db: &PgConnection, id: i32) -> Result<Project, Error> {
        use super::super::schema::projects::dsl::{ projects, deleted_at };
        projects.filter(deleted_at.is_null()).find::<i32>(id.into()).first(db)
    }
    fn all(db: &PgConnection) -> Result<Vec<Project>, Error> {
        use super::super::schema::projects::dsl::{ projects, deleted_at };
        projects.filter(deleted_at.is_null()).get_results(db)
    }
    fn update(self, db: &PgConnection) -> Result<Project, Error> {
        self.save_changes::<Project>(db)
    }
    fn delete(mut self, db: &PgConnection) -> Result<Project, Error>{
        self.deleted_at = Option::Some(chrono::offset::Local::now().naive_local());
        self.save_changes::<Project>(db)
    }
}

impl Project{
    pub fn by_category(db: &PgConnection, category_id: i32) -> Result<Vec<Project>, Error>{
        use super::super::schema::projects::dsl::{ projects, deleted_at, category_id };
        projects.filter(deleted_at.is_null()).filter(category_id.eq(category_id)).get_results(db)
    }
}

impl NewModel<Project> for NewProject {
    fn save(self, db: &PgConnection) -> Result<Project, Error> {
        use super::super::schema::projects::dsl::{ projects };
        diesel::insert_into(projects)
            .values(&self)
            .get_result(db)
    }
}

impl NewProject {
    pub fn check_slug_unique(self, slug_to_find: String, db: &PgConnection)-> bool {
        use super::super::schema::projects::dsl::{ projects, slug };
        projects.filter(slug.eq(slug_to_find)).first::<Project>(db).is_err()
    }
}
