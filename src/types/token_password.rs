use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct TokenPassword {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_expires_in: u64,
    pub refresh_token: String,
    pub token_type: String,
    pub id_token: String,
    #[serde(rename = "not-before-policy")]
    pub not_before_policy: i32,
    pub session_state: String, 
    pub scope: String,
}
