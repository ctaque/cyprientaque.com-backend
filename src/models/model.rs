use std::env;
use async_trait::async_trait;
use tokio_postgres::{ Client, NoTls, Error};
use tokio;

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
    async fn all() -> Result<Vec<T>, Error>;
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
