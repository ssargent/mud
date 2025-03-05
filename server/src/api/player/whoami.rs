use axum::{response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};

use crate::api::auth::CurrentUser;

#[derive(Serialize, Deserialize)]
struct UserResponse {
    id: i64,
    email: String,
    full_name: String,
}

pub async fn player_whoami(Extension(currentUser): Extension<CurrentUser>) -> impl IntoResponse {
    Json(UserResponse {
        id: currentUser.id,
        email: currentUser.email,
        full_name: currentUser.full_name,
    })
}
