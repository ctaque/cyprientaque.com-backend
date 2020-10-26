use crate::services::bitbucket;
use actix_web::{ web, HttpResponse };
use serde_json::json;

pub struct Bitbucket;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RefreshTokenQuery {
    refresh_token: String,
}

impl Bitbucket {
    pub async fn access_token() -> Result<HttpResponse, HttpResponse> {
        let resp = bitbucket::get_access_token().await;
        match resp {
            Ok(token) => Ok(HttpResponse::Ok().body(json!(token))),
            Err(err) => Err(HttpResponse::Unauthorized().body(err.to_string())),
        }
    }

    pub async fn refresh_token(info: web::Query<RefreshTokenQuery>) -> Result<HttpResponse, HttpResponse> {
        let resp = bitbucket::refresh_token(info.refresh_token.to_string()).await;
        match resp {
            Ok(token) => Ok(HttpResponse::Ok().body(json!(token))),
            Err(err) => Err(HttpResponse::Unauthorized().body(err.to_string())),
        }
    }
}
