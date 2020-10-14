use postgres::{ Row };
use async_trait::async_trait;
use postgres::{ error::Error };
use chrono::naive::NaiveDateTime;
use rest_macro_derive::{HttpAll, HttpFind, HttpDelete };
use rest_macro::{HttpAll, HttpFind, HttpDelete, DeleteInfo, FindInfo, Model };
use actix_web::{ HttpResponse, web };
use serde_json::json;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, HttpFind, HttpAll, HttpDelete)]
pub struct ProjectImageCategory {
    pub id: i32,
    pub name: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl ProjectImageCategory{
    pub fn new(row: &Row) -> Self{
        ProjectImageCategory{
            id: row.get("id"),
            name: row.get("name"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            deleted_at: row.get("deleted_at"),
        }
    }
}

#[async_trait]
impl Model<ProjectImageCategory> for ProjectImageCategory {
    async fn find(id: i32) -> Result<ProjectImageCategory, Error>
    where ProjectImageCategory: 'async_trait{
        let row: Row = Self::db().await.query_one("select * from project_image_categories where id = $1;",  &[&id]).await?;
        let c = ProjectImageCategory::new(&row);
        Ok(c)
    }
    async fn all() -> Result<Vec<ProjectImageCategory>, Error>
        where ProjectImageCategory: 'async_trait{
        let rows: Vec<Row> = Self::db().await.query("select * from project_image_categories;", &[]).await?;
        let mut categories = Vec::new();
        for row in rows{
            let p = ProjectImageCategory::new(&row);
            categories.push(p);
        }
        Ok(categories)
    }
    async fn update(self) -> Result<ProjectImageCategory, Error> {

        let row: Row = Self::db().await.query_one("update project_image_categories set name = $2, updated_at = CURRENT_TIMESTAMP where id = $1 returning *;",
                                    &[&self.id, &self.name]).await?;
        let c = ProjectImageCategory::new(&row);
        Ok(c)

    }
    async fn delete(mut self) -> Result<ProjectImageCategory, Error>{
        let row = Self::db().await.query_one("update project_image_categories set deleted_at = CURRENT_TIMESTAMP where id = $1 returning *", &[&self.id]).await?;
        let c = ProjectImageCategory::new(&row);
        Ok(c)
    }
}
