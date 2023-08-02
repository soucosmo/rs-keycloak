use serde::Deserialize;

#[derive(Deserialize)]
pub struct TokenClient {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_expires_in: u64,
    pub token_type: String,
    #[serde(rename = "not-before-policy")]
    pub not_before_policy: i32,
    pub scope: String,
}
