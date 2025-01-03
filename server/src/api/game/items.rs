use axum::{http::StatusCode, Json};
use protocol::TypeSignature;

use crate::api::{ApiResponse, Payload};
use crate::{app_state::AppState, game::Item, ItemRepository};
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

pub async fn get_world_item_by_code(
    State(state): State<AppState>,
    axum::extract::Path((world_code, item_code)): axum::extract::Path<(String, String)>,
) -> ApiResponse<Item> {
    let pool = state.db_pool.clone();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::Error("Failed to get connection".to_string()),
    };

    // todo: find world by code
    // todo: find item by code and world id

    match ItemRepository::find_item_by_code(&mut conn, 1, &item_code) {
        Ok(item) => ApiResponse::JsonData(Payload { data: item }),
        Err(_) => ApiResponse::NotFound,
    }
}

pub async fn get_item(
    State(state): State<AppState>,
    axum::extract::Path(item_id): axum::extract::Path<i64>,
) -> ApiResponse<Item> {
    let pool = state.db_pool.clone();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::Error("Failed to get connection".to_string()),
    };

    match ItemRepository::find_by_id(&mut conn, item_id) {
        Ok(item) => ApiResponse::JsonData(Payload { data: item }),
        Err(_) => ApiResponse::NotFound,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDefinition {
    pub id: Option<i64>,
    pub world_id: i64,
    pub category_id: i64,
    pub code: String,
    pub item_type: String,
    pub name: String,
    pub description: String,
    pub item_properties: serde_json::Value,
    pub base_price: i64,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl TypeSignature for ItemDefinition {
    fn signature(&self) -> Vec<u8> {
        let mut signature = Vec::new();
        signature.extend_from_slice(&self.world_id.to_be_bytes());
        signature.extend_from_slice(&self.category_id.to_be_bytes());
        signature.extend_from_slice(self.code.as_bytes());
        signature.extend_from_slice(self.item_type.as_bytes());
        signature.extend_from_slice(self.name.as_bytes());
        signature.extend_from_slice(self.description.as_bytes());
        signature.extend_from_slice(&self.base_price.to_be_bytes());

        Self::as_hashed(signature)
    }
}

impl ItemDefinition {
    pub fn to_item(&self) -> Item {
        Item {
            id: self.id.unwrap_or(0),
            world_id: self.world_id,
            category_id: self.category_id,
            code: self.code.clone(),
            item_type: self.item_type.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            item_properties: self.item_properties.clone(),
            base_price: self.base_price,
            created_at: self.created_at.unwrap_or(chrono::Utc::now().naive_utc()),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.world_id > 0
            && self.category_id > 0
            && !self.code.is_empty()
            && !self.item_type.is_empty()
            && !self.name.is_empty()
            && !self.description.is_empty()
            && self.base_price > 0
    }
}

pub async fn create_item(
    State(state): State<AppState>,
    Json(item): Json<ItemDefinition>,
) -> ApiResponse<Item> {
    // todo: More information as to what is wrong with the item definition
    if !item.is_valid() {
        return ApiResponse::BadRequest("Invalid item definition".to_string());
    }

    let pool = state.db_pool.clone();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::Error("Failed to get connection".to_string()),
    };

    // look up an item by its code and world id.  If it exists, compare the signature
    let found_item = ItemRepository::find_item_by_code(&mut conn, item.world_id, &item.code);
    // if the item is found, compare the signatures
    // if it is the same, return NotChanged
    // if it is not found or different, then just create or update the item.
    if let Ok(found_item) = found_item {
        if item.signature() == found_item.signature() {
            return ApiResponse::NotChanged;
        }
    }

    let data_item = Item {
        id: item.id.unwrap_or(0),
        world_id: item.world_id,
        category_id: item.category_id,
        code: item.code.clone(),
        item_type: item.item_type.clone(),
        name: item.name.clone(),
        description: item.description.clone(),
        item_properties: item.item_properties.clone(),
        base_price: item.base_price,
        created_at: item.created_at.unwrap_or(chrono::Utc::now().naive_utc()),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    match ItemRepository::create_or_update(&mut conn, &data_item) {
        Ok(item) => {
            if data_item.id == 0 {
                ApiResponse::Created(Payload { data: item })
            } else {
                ApiResponse::JsonData(Payload { data: item })
            }
        }
        Err(_) => ApiResponse::Error("Failed to create item".to_string()),
    }
}
