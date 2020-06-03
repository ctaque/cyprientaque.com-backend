// use chrono::Utc;
// use jsonwebtoken::{
//     EncodingKey,
//     Header
// };

// static ONE_MINUTE: i64 = 60; // in seconds

#[derive(serde::Serialize, serde::Deserialize)]
pub struct JWTToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user_id: i32,
}

// impl JWTToken {
//     pub fn generate_token(user_id: i32, encoding_key: String) -> String {
//         let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
//         let payload = UserToken {
//             iat: now,
//             exp: now + ONE_MINUTE,
//             user_id,
//         };

//         jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_base64_secret(encoding_key)).expect("Error encoding JWT")
//     }
// }
