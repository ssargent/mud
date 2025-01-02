pub mod rest;

use crate::{
    api::rest::create_item, api::rest::get_item, api::rest::get_world_item_by_code,
    app_state::AppState,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub fn rest_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/:world_code/items/:item_code",
            axum::routing::get(get_world_item_by_code),
        )
        .route("/items/:id", axum::routing::get(get_item))
        .route("/items", axum::routing::put(create_item))
}
