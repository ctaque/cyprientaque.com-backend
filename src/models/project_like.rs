use chrono::naive::NaiveDateTime;
use postgres::{ Row, error::Error };
use tokio_postgres::{ Client, NoTls };
use std::{env};
use ipnetwork::IpNetwork;
use std::str::FromStr;
use std::net::IpAddr;


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ProjectLike{
    pub id: i32,
    pub project_id: i32,
    pub created_at: NaiveDateTime,
    pub ip: IpNetwork,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NewProjectLike{
    pub project_id: i32,
    pub ip: IpNetwork,
}

impl ProjectLike{
    async fn db() -> Client {
        let (client, connection) =
            tokio_postgres::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"), NoTls).await.expect(&format!("Error connecting to {}", env::var("DATABASE_URL").unwrap()));

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        client
    }
    pub fn new(row: &Row) -> Self{
        let ip: IpAddr = row.get("ip");
        ProjectLike{
            id: row.get("id"),
            created_at: row.get("created_at"),
            project_id: row.get("project_id"),
            ip: IpNetwork::from_str(&ip.to_string()).expect("Invalid Ip Adress")
        }
    }

    pub async fn get_count_for_project(project_id: i64) -> Result<i64, Error> {
        let row: Row = Self::db().await.query_one(
            "SELECT count(id) as count FROM projects_likes where project_id = $1",
            &[&project_id]
        ).await?;
        let count: i64 = row.get("count");
        Ok(count)
    }
}

impl NewProjectLike{
    pub async fn toggle(self) -> Result<Option<ProjectLike>, Error> {
        let result_count_row: Result<Row, Error> = ProjectLike::db().await.query_one("SELECT count(id) as count from projects_likes where project_id = $1 AND ip = $2::text::inet", &[&self.project_id, &self.ip.to_string()]).await;
        println!("ok");
        let count: i64 = if let Ok(count_row) = result_count_row{
            count_row.get("count")
        } else {
            0i64
        };
        if count == 1 {
            Self::delete(self.project_id, self.ip).await?;
            Ok(None)
        } else {
            let row: Row = ProjectLike::db().await.query_one(
                "INSERT INTO projects_likes (project_id, created_at, ip) VALUES ($1, CURRENT_TIMESTAMP, $2::text::inet) returning *;",
                &[&self.project_id, &self.ip.to_string()]
            ).await?;
            Ok(Some(ProjectLike::new(&row)))
        }
    }

    pub async fn delete(project_id: i32, ip: IpNetwork) -> Result<(), Error> {
        ProjectLike::db().await.query(
            "DELETE FROM projects_likes WHERE project_id = $1 AND ip = $2::text::inet",
            &[&project_id, &ip.to_string()]
        ).await?;
        Ok(())
    }

}
