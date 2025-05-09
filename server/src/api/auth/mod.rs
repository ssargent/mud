use crate::app_state::AppState;
use axum::Router;
use axum::{middleware, routing::post};

mod external;
mod login;
mod types;

use external::auth_external_login_apikey;
use login::{auth_create_api_key, auth_register};
pub use login::{auth_login, authorize, CurrentUser};
pub use types::{ExternalLogin, LoginResult};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(auth_login))
        //    .route("/auth/logout", post(auth_logout))
        .route("/auth/register", post(auth_register))
        .route(
            "/auth/api_key",
            post(auth_create_api_key).layer(middleware::from_fn(|req, next| {
                super::auth::authorize(req, next)
            })),
        )
}
