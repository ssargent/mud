pub mod character_classes;
pub mod items;
pub mod worlds;

use crate::app_state::AppState;
use axum::routing::{get, post, put};
use axum::Router;
pub use character_classes::{create_or_update_character_class, get_character_class_by_code};
pub use items::{create_item, get_item, get_world_item_by_code};
pub use worlds::{create_new_game_world, create_or_update_game_world, get_world_by_code};

pub fn game_routes() -> Router<AppState> {
    Router::new()
        .route("/game/:world_code", get(get_world_by_code))
        .route("/game", post(create_new_game_world))
        .route("/game/:world_code", put(create_or_update_game_world))
        .route(
            "/game/:world_code/items/:item_code",
            get(get_world_item_by_code),
        )
        .route("/game/:world_code/items/:item_code", post(create_item))
        .route("/game/:world_code/items/:item_code", put(create_item))
        .route(
            "/game/:world_code/classes/:class_code",
            put(create_or_update_character_class),
        )
        .route(
            "/game/:world_code/classes/:class_code",
            get(get_character_class_by_code),
        )
        // legacy routes
        .route("/game/items/:id", axum::routing::get(get_item))
        .route("/game/items", axum::routing::put(create_item))
}
