use reqwest;


#[derive(serde::Serialize)]
struct GrantAccess{
    grant_type: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AccessTokenResponse{
    pub scopes: String,
    pub access_token: String,
    pub expires_in: u32,
    pub token_type: String,
    pub state: String,
    pub refresh_token: String,
}

pub async fn get_access_token()-> Result<AccessTokenResponse, reqwest::Error>{
    let client = reqwest::Client::new();
    let url = "https://bitbucket.org/site/oauth2/access_token";
    let resp = client.post(url)
        .basic_auth("NX52hHDHnJpkJutLWM", Option::Some("SdCgmzQZUpRWA2cSKg5jLjrHRC56zJu3"))
        .form(&GrantAccess{ grant_type: "client_credentials".to_string() })
        .send()
        .await.unwrap();
    resp.json::<AccessTokenResponse>().await
}

#[derive(serde::Serialize)]
struct GrantRefresh{
    grant_type: String,
    refresh_token: String,
}

pub async fn refresh_token(refresh_token: String) -> Result<AccessTokenResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = "https://bitbucket.org/site/oauth2/access_token";
    let resp = client.post(url)
        .basic_auth("NX52hHDHnJpkJutLWM", Option::Some("SdCgmzQZUpRWA2cSKg5jLjrHRC56zJu3"))
        .form(&GrantRefresh{ grant_type: "refresh_token".to_string(), refresh_token: refresh_token})
        .send()
        .await.unwrap();
    resp.json::<AccessTokenResponse>().await
}
