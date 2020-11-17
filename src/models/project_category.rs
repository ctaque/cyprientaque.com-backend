use postgres::{ Row };
use async_trait::async_trait;
use postgres::{ error::Error };
use chrono::naive::NaiveDateTime;
use rest_macro_derive::{HttpAll, HttpFind, HttpDelete };
use rest_macro::{HttpAll, HttpFind, FindInfo, HttpDelete, DeleteInfo, Model };
use actix_web::{ HttpResponse, web };
use serde_json::json;

pub enum ProjectCategoryHardcoded {
    Web,
    Ebenisterie,
    Nautisme,
    Mobilier,
    Blog,
}

impl ProjectCategoryHardcoded {
    pub fn value(self) -> i32{
        match self{
            ProjectCategoryHardcoded::Web => 1,
            ProjectCategoryHardcoded::Ebenisterie => 2,
            ProjectCategoryHardcoded::Mobilier => 3,
            ProjectCategoryHardcoded::Nautisme => 4,
            ProjectCategoryHardcoded::Blog => 5
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, HttpFind, HttpAll, HttpDelete)]
pub struct ProjectCategory {
    pub id: i32,
    pub name: String,
    pub picture_url: Option<String>,
    pub slug: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub color_hex: String,
    pub project_count: Option<i64>
}

impl ProjectCategory{
    pub fn new(row: &Row) -> Self{
        ProjectCategory{
            id: row.get("id"),
            name: row.get("name"),
            picture_url: row.get("picture_url"),
            slug: row.get("slug"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            deleted_at: row.get("deleted_at"),
            color_hex: row.get("color_hex"),
            project_count: None,
        }
    }
    async fn attach_project_count(mut self) -> Result<ProjectCategory, Error> {
        let row: Row = Self::db().await.query_one("SELECT count(id) as project_count from projects where category_id = $1", &[&self.id]).await?;
        self.project_count = row.get("project_count");
        Ok(self)
    }

    pub fn pretty_print(self) -> (){
        println!(
            "Category id: {}, name: {}, color hex: {}, picture url: {}, created_at: {}, updated_at: {}, deleted_at: {}",
            self.id,
            self.name,
            self.picture_url.unwrap_or("null".to_string()),
            self.color_hex,
            self.created_at.to_string(),
            self.updated_at.to_string(),
            self.deleted_at.and_then(|date| Some(date.to_string())).unwrap_or("null".to_string()),
        );
    }

    pub async fn print_all() -> Result<(), String> {
        let result = Self::all().await;
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

    pub fn selectify_categories(categories: &Vec<ProjectCategory>) -> Vec<String> {
        categories.into_iter().map(| x | format!("id: {}, name: {}", x.id, x.name)).collect()
    }
}

#[async_trait]
impl Model<ProjectCategory> for ProjectCategory {
    async fn find(id: i32) -> Result<ProjectCategory, Error>
    where ProjectCategory: 'async_trait{
        let row: Row = Self::db().await.query_one("select * from project_categories where id = $1 and deleted_at is null;",  &[&id]).await?;
        let c = ProjectCategory::new(&row);
        Ok(c)
    }
    async fn all() -> Result<Vec<ProjectCategory>, Error>
        where ProjectCategory: 'async_trait{
        let rows: Vec<Row> = Self::db().await.query("select * from project_categories;", &[]).await?;
        let mut categories = Vec::new();
        for row in rows{
            let p = ProjectCategory::new(&row);
            let p = p.attach_project_count().await?;
            categories.push(p);
        }
        Ok(categories)
    }
    async fn update(self) -> Result<ProjectCategory, Error> {

        let row: Row = Self::db().await.query_one("update project_categories set name = $2, picture_url = $3, slug = $4, updated_at = CURRENT_TIMESTAMP where id = $1 returning *;",
                                    &[&self.id, &self.name, &self.picture_url, &self.slug]).await?;
        let c = ProjectCategory::new(&row);
        Ok(c)

    }
    async fn delete(mut self) -> Result<ProjectCategory, Error>{
        let row = Self::db().await.query_one("update project_categories set deleted_at = CURRENT_TIMESTAMP where id = $1 returning *", &[&self.id]).await?;
        let c = ProjectCategory::new(&row);
        Ok(c)
    }
}
