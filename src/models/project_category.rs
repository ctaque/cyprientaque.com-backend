use postgres::{ Row };
use crate::models::model::Model;
use async_trait::async_trait;
use postgres::{ error::Error };
use chrono::naive::NaiveDateTime;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
        let row = Self::db().await.query_one("update project_categories set deleted_at = CURRENT_TIMESTAMP where id = $1", &[&self.id]).await?;
        let c = ProjectCategory::new(&row);
        Ok(c)
    }
}

impl ProjectCategory{
    async fn attach_project_count(mut self) -> Result<ProjectCategory, Error> {
        let row: Row = Self::db().await.query_one("SELECT count(id) as project_count from projects where category_id = $1", &[&self.id]).await?;
        self.project_count = row.get("project_count");
        Ok(self)
    }
}
