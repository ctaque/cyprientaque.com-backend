use async_trait::async_trait;
use actix_web::{ HttpResponse, web };
use std::env;
use tokio_postgres::{ Client, NoTls, Error};
use tokio;
use serde::{Deserialize};

#[async_trait]
pub trait Model<T> {
    async fn db() -> Client
    where T: 'async_trait{
        let (client, connection) =
            tokio_postgres::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"), NoTls).await.expect(&format!("Error connecting to {}", env::var("DATABASE_URL").unwrap()));

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        client
    }
    async fn find(id: i32) -> Result<T, Error>;
    async fn all(query: HttpAllOptionalQueryParams) -> Result<Vec<T>, Error>;
    async fn update(self: Self) -> Result<T, Error>;
    async fn delete(self: Self) -> Result<T, Error>;
}

#[async_trait]
pub trait NewModel<T> {
    async fn db() -> Client
        where T: 'async_trait{
        let (client, connection) =
            tokio_postgres::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"), NoTls).await.expect(&format!("Error connecting to {}", env::var("DATABASE_URL").unwrap()));

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        client
    }
    async fn save(self: Self) -> Result<T, Error>;
}

#[async_trait]
pub trait UpdatableModel<T> {
    async fn db() -> Client
    where T: 'async_trait{
        let (client, connection) =
            tokio_postgres::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"), NoTls).await.expect(&format!("Error connecting to {}", env::var("DATABASE_URL").unwrap()));

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        client
    }
    async fn update(self: Self) -> Result<T, Error>;
}

#[derive(Deserialize, Default)]
pub struct HttpAllOptionalQueryParams {
    pub author: Option<bool>,
    pub category: Option<bool>,
    pub images: Option<bool>,
    pub primary_image: Option<bool>,
}

impl HttpAllOptionalQueryParams {
    pub fn attach_author(&self) -> bool{
        if let Some(_) = self.author {
            return true;
        };
        false
    }
    pub fn attach_category(&self) -> bool{
        if let Some(_) = self.category {
           return true;
        };
        false
    }
    pub fn attach_images(&self) -> bool{
        if let Some(_) = self.images {
            return true;
        };
        false
    }
    pub fn attach_primary_image(&self) -> bool{
        if let Some(_) = self.primary_image {
            return true;
        };
        false
    }
}

#[async_trait]
pub trait HttpAll {
    async fn http_all(query: web::Query<HttpAllOptionalQueryParams>) -> Result<HttpResponse, HttpResponse>;
}

#[derive(Deserialize)]
pub struct FindInfo{
    pub id: i32,
}
#[async_trait]
pub trait HttpFind {
    async fn http_find(info: web::Path<FindInfo>) -> Result<HttpResponse, HttpResponse>;
}


#[derive(Deserialize)]
pub struct DeleteInfo{
    pub id: i32,
}
#[async_trait]
pub trait HttpDelete {
    async fn http_delete(info: web::Path<DeleteInfo>) -> Result<HttpResponse, HttpResponse>;
}
