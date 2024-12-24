use axum::{http::StatusCode, Json};

use crate::{app_state::AppState, game::Item, ItemRepository};
use axum::extract::State;

pub async fn get_item(
    State(state): State<AppState>,
    axum::extract::Path(item_id): axum::extract::Path<i64>,
) -> Result<Json<Item>, StatusCode> {
    let pool = state.db_pool.clone();
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    ItemRepository::find_by_id(&mut conn, item_id)
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

pub async fn create_item(
    State(state): State<AppState>,
    Json(item): Json<Item>,
) -> Result<Json<Item>, StatusCode> {
    let pool = state.db_pool.clone();
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    ItemRepository::create_or_update(&mut conn, &item)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
