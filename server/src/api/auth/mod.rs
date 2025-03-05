use crate::app_state::AppState;
use axum::routing::{get, post, put};
use axum::Router;

mod login;

use login::auth_register;
pub use login::{auth_login, authorize, CurrentUser};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(auth_login))
        //    .route("/auth/logout", post(auth_logout))
        .route("/auth/register", post(auth_register))
}
