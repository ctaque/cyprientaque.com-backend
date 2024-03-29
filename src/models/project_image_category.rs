use postgres::{ Row };
use async_trait::async_trait;
use postgres::{ error::Error };
use chrono::naive::NaiveDateTime;
use rest_macro_derive::{HttpAll, HttpFind, HttpDelete };
use rest_macro::{HttpAll, HttpAllOptionalQueryParams, HttpFind, HttpDelete, DeleteInfo, FindInfo, Model };
use actix_web::{ HttpResponse, web };
use serde_json::json;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, HttpFind, HttpAll, HttpDelete)]
pub struct ProjectImageCategory {
    pub id: i32,
    pub name: String,
    pub color_hex: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl ProjectImageCategory{
    pub fn new(row: &Row) -> Self{
        ProjectImageCategory{
            id: row.get("id"),
            name: row.get("name"),
            color_hex: row.get("color_hex"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            deleted_at: row.get("deleted_at"),
        }
    }
    pub fn pretty_print(self) -> (){
        println!(
            "Image category id: {}, name: {}, color hex: {}, created_at: {}, updated_at: {}, deleted_at: {}",
            self.id,
            self.name,
            self.color_hex,
            self.created_at.and_then(|date| Some(date.to_string())).unwrap_or("null".to_string()),
            self.updated_at.and_then(|date| Some(date.to_string())).unwrap_or("null".to_string()),
            self.deleted_at.and_then(|date| Some(date.to_string())).unwrap_or("null".to_string()),
        );
    }

    pub async fn print_all() -> Result<(), String> {
        let options: HttpAllOptionalQueryParams = Default::default();
        let result = Self::all(options).await;
        match result {
            Ok(cats) => {
                for cat in cats {
                    cat.pretty_print();
                };
                Ok(())
            },
            Err(err) => Err(err.to_string())
        }
    }

    pub fn selectify(categories: &Vec<ProjectImageCategory>) -> Vec<String> {
        categories.into_iter().map(| x | format!("id: {}, name: {}", x.id, x.name)).collect()
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
    async fn all(_params: HttpAllOptionalQueryParams) -> Result<Vec<ProjectImageCategory>, Error>
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
