use axum::Json;
use protocol::TypeSignature;

use crate::api::{ApiResponse, Payload};
use crate::{app_state::AppState, game::World, WorldRepository};
use axum::extract::{Path, State};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldDefinition {
    pub id: Option<i64>,
    pub code: String,
    pub name: String,
    pub description: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl TypeSignature for WorldDefinition {
    fn signature(&self) -> Vec<u8> {
        let mut signature = Vec::new();
        signature.extend_from_slice(self.code.as_bytes());
        signature.extend_from_slice(self.name.as_bytes());
        signature.extend_from_slice(self.description.as_bytes());

        Self::as_hashed(signature)
    }
}

impl WorldDefinition {
    pub fn to_world(&self) -> World {
        World {
            id: self.id.unwrap_or(0),
            code: self.code.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            created_at: self
                .created_at
                .unwrap_or_else(|| chrono::Utc::now().naive_utc()),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.code.is_empty() && !self.name.is_empty() && !self.description.is_empty()
    }
}

pub async fn get_world_by_code(
    State(state): State<AppState>,
    Path(world_code): Path<String>,
) -> ApiResponse<World> {
    let pool = state.db_pool.clone();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::Error("Failed to get connection".to_string()),
    };

    let world = WorldRepository::find_by_code(&mut conn, &world_code);
    match world {
        Ok(world) => ApiResponse::JsonData(Payload { data: world }),
        Err(_) => ApiResponse::NotFound("World not found".to_string()),
    }
}

pub async fn create_new_game_world(
    State(state): State<AppState>,
    Json(world): Json<WorldDefinition>,
) -> ApiResponse<World> {
    let pool = state.db_pool.clone();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::Error("Failed to get connection".to_string()),
    };

    if !world.is_valid() {
        return ApiResponse::BadRequest(vec!["Invalid world definition".to_string()]);
    }

    if let Ok(_) = WorldRepository::find_by_code(&mut conn, &world.code) {
        return ApiResponse::BadRequest(vec!["World already exists".to_string()]);
    }

    match WorldRepository::create(&mut conn, &world.to_world().as_new_world()) {
        Ok(world) => ApiResponse::JsonData(Payload { data: world }),
        Err(err) => ApiResponse::Error(err.to_string()),
    }
}

pub async fn create_or_update_game_world(
    State(state): State<AppState>,
    Path(world_code): Path<String>,
    Json(world): Json<WorldDefinition>,
) -> ApiResponse<World> {
    let pool = state.db_pool.clone();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::Error("Failed to get connection".to_string()),
    };

    if !world.is_valid() {
        return ApiResponse::BadRequest(vec!["Invalid world definition".to_string()]);
    }

    if world.code != world_code {
        return ApiResponse::BadRequest(vec!["World code mismatch".to_string()]);
    }

    let found_world: Option<World> = match WorldRepository::find_by_code(&mut conn, &world.code) {
        Ok(found_world) => {
            if world.signature() == found_world.signature() {
                return ApiResponse::NotChanged;
            }
            Some(found_world)
        }
        Err(_) => None,
    };

    let world_to_create = World {
        id: found_world.map(|w| w.id).unwrap_or(0),
        code: world.code.clone(),
        name: world.name.clone(),
        description: world.description.clone(),
        created_at: world
            .created_at
            .unwrap_or_else(|| chrono::Utc::now().naive_utc()),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    match WorldRepository::create_or_update(&mut conn, &world_to_create) {
        Ok(world) => ApiResponse::JsonData(Payload { data: world }),
        Err(err) => ApiResponse::Error(err.to_string()),
    }
}
