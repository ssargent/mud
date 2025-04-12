use axum::{response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};

use crate::api::auth::CurrentUser;

#[derive(Serialize, Deserialize)]
struct UserResponse {
    id: i64,
    email: String,
    full_name: String,
}

pub async fn player_whoami(Extension(current_user): Extension<CurrentUser>) -> impl IntoResponse {
    Json(UserResponse {
        id: current_user.id,
        email: current_user.email,
        full_name: current_user.full_name,
    })
}
