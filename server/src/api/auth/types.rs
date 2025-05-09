use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResult {
    pub token: String,
    pub status: u16,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalLogin {
    pub token: String,
    pub scopes: Vec<String>,
}
