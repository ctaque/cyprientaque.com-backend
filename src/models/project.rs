use chrono::naive::NaiveDateTime;
use diesel::result::Error;
use crate::models::model::Model;
use diesel::pg::PgConnection;
use super::super::diesel::prelude::*;
use crate::schema::projects;

#[derive(Queryable, AsChangeset, serde::Serialize, serde::Deserialize)]
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
}

impl Model<Project> for Project {
    fn find(db: &PgConnection, id: i32) -> Result<Project, Error> {
        use super::super::schema::projects::dsl::{ projects, deleted_at };
        projects.filter(deleted_at.is_null()).find::<i32>(id.into()).first(db)
    }
    fn save(self, db: &PgConnection) -> Result<Project, Error> {
        Ok(self)
    }
    fn update(self, db: &PgConnection) -> Result<Project, Error> {
        use super::super::schema::projects::dsl::{ projects, id,  deleted_at };
        diesel::update(projects.filter(id.eq(self.id)).or_filter(deleted_at.is_null()))
            .set::<Project>(self.into())
            .get_result(db)
    }
    fn delete(self, db: &PgConnection) -> Result<(), Error>{
        Ok(())
    }
}

