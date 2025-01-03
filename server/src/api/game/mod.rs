pub mod items;

use crate::app_state::AppState;
use axum::Router;
pub use items::{create_item, get_item, get_world_item_by_code};

pub fn game_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/game/:world_code/items/:item_code",
            axum::routing::get(get_world_item_by_code),
        )
        .route(
            "/game/:world_code/items/:item_code",
            axum::routing::post(create_item),
        )
        .route(
            "/game/:world_code/items/:item_code",
            axum::routing::put(create_item),
        )
        // legacy routes
        .route("/game/items/:id", axum::routing::get(get_item))
        .route("/game/items", axum::routing::put(create_item))
}
