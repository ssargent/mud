use axum::{Json, extract::State, http::StatusCode};

use crate::{
    api::{
        ApiResponse, Payload,
        auth::{CurrentUser, login::encode_jwt},
    },
    app_state::AppState,
    db::{
        SystemUserRepository,
        system::{ActiveUserRole, NewUser, NewUserApiKey},
    },
}; // Adjust the path to where ApiResponse is defined

use super::{ExternalLogin, LoginResult};

pub async fn auth_external_login_apikey(
    State(state): State<AppState>,
    Json(login): Json<ExternalLogin>,
) -> ApiResponse<Option<LoginResult>> {
    // Validate the token and scopes
    if login.token.is_empty() || login.scopes.is_empty() {
        return ApiResponse::BadRequest(vec!["Token and scopes cannot be empty".to_string()]);
    }

    let pool = state.db_pool.clone();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::Error("Failed to get connection".to_string()),
    };

    let user = match SystemUserRepository::get_user_by_api_key(&mut conn, &login.token) {
        Ok(Some(user)) => user,
        Ok(None) => return ApiResponse::NotFound("User not found (404)".to_string()),
        Err(err) => return ApiResponse::Error(format!("Database error: {}", err)),
    };

    let full_user_permissions = match SystemUserRepository::get_user_permissions(&mut conn, user.id)
    {
        Ok(permissions) => permissions,
        Err(err) => return ApiResponse::Error(format!("Database error: {}", err)),
    };

    if !full_user_permissions.contains(&"login.external.apikey".to_string()) {
        return ApiResponse::Unauthorized(
            "User does not have permission to login with API key".to_string(),
        );
    };

    // check the scopes, and filter them by the permissions
    let filtered_scopes: Vec<String> = login
        .scopes
        .into_iter()
        .filter(|scope| full_user_permissions.contains(scope))
        .collect();

    let cu = CurrentUser {
        id: user.id,
        email: user.email,
        full_name: user.full_name,
        permissions: Some(filtered_scopes),
    };

    match encode_jwt(cu, vec![], vec![], Some("api-login".to_string())) {
        Ok(token) => ApiResponse::JsonData(Payload {
            data: Some(LoginResult {
                token: token.clone(),
                status: StatusCode::OK.as_u16(),
                message: "Login successful".to_string(),
            }),
        }),
        Err(_) => ApiResponse::Error("Failed to create token".to_string()),
    }
}
