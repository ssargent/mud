use super::auth::authorize;
use crate::app_state::AppState;
use axum::routing::{get, post, put};
use axum::{middleware, Router};
use whoami::player_whoami;

mod whoami;

pub fn player_routes() -> Router<AppState> {
    Router::new().route(
        "/player/whoami",
        get(player_whoami).layer(middleware::from_fn(super::auth::authorize)),
    )
}
