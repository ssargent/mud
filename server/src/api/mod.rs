pub mod rest;

use crate::{api::rest::create_item, api::rest::get_item, app_state::AppState};
use axum::Router;

pub fn rest_routes() -> Router<AppState> {
    Router::new()
        .route("/items/:id", axum::routing::get(get_item))
        .route("/items", axum::routing::put(create_item))
}
