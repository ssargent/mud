use axum::Json;
use protocol::TypeSignature;

use crate::api::{ApiResponse, Payload};
use crate::{app_state::AppState, game::Enemy, game::NewEnemy, EnemyRepository, WorldRepository};
use axum::extract::{Path, State};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyDefinition {
    pub id: Option<i64>,
    pub world_id: Option<i64>,
    pub code: String,
    pub name: String,
    pub description: String,
    pub class: String,
    pub level: i32,
    pub hit_points: i32,
    pub stamina: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub weapons: serde_json::Value,
    pub armor: serde_json::Value,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl TypeSignature for EnemyDefinition {
    fn signature(&self) -> Vec<u8> {
        let mut signature = Vec::new();
        if let Some(world_id) = self.world_id {
            signature.extend_from_slice(&world_id.to_be_bytes());
        }
        signature.extend_from_slice(self.code.as_bytes());
        signature.extend_from_slice(self.name.as_bytes());
        signature.extend_from_slice(self.description.as_bytes());
        signature.extend_from_slice(self.class.as_bytes());
        signature.extend_from_slice(&self.level.to_be_bytes());
        signature.extend_from_slice(&self.hit_points.to_be_bytes());
        signature.extend_from_slice(&self.stamina.to_be_bytes());
        signature.extend_from_slice(&self.strength.to_be_bytes());
        signature.extend_from_slice(&self.dexterity.to_be_bytes());
        signature.extend_from_slice(&self.constitution.to_be_bytes());
        signature.extend_from_slice(&self.intelligence.to_be_bytes());
        signature.extend_from_slice(&self.wisdom.to_be_bytes());
        signature.extend_from_slice(self.weapons.to_string().as_bytes());
        signature.extend_from_slice(self.armor.to_string().as_bytes());

        Self::as_hashed(signature)
    }
}

impl EnemyDefinition {
    pub fn to_enemy(&self) -> Enemy {
        Enemy {
            id: self.id.unwrap_or(0),
            world_id: self.world_id.unwrap_or(0),
            code: self.code.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            class: self.class.clone(),
            level: self.level,
            hit_points: self.hit_points,
            stamina: self.stamina,
            strength: self.strength,
            dexterity: self.dexterity,
            constitution: self.constitution,
            intelligence: self.intelligence,
            wisdom: self.wisdom,
            weapons: self.weapons.clone(),
            armor: self.armor.clone(),
            created_at: self.created_at.unwrap_or(chrono::Utc::now().naive_utc()),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn is_valid(&self) -> Result<(), Vec<String>> {
        let mut errors: Vec<String> = vec![];

        if self.code.is_empty() {
            errors.push("Code is required".to_string());
        }

        if self.name.is_empty() {
            errors.push("Name is required".to_string());
        }

        if self.description.is_empty() {
            errors.push("Description is required".to_string());
        }

        if self.class.is_empty() {
            errors.push("Class is required".to_string());
        }

        if self.level <= 0 {
            errors.push("Level must be greater than 0".to_string());
        }

        if self.hit_points <= 0 {
            errors.push("Hit points must be greater than 0".to_string());
        }

        if self.stamina <= 0 {
            errors.push("Stamina must be greater than 0".to_string());
        }

        if self.strength <= 0 {
            errors.push("Strength must be greater than 0".to_string());
        }

        if self.dexterity <= 0 {
            errors.push("Dexterity must be greater than 0".to_string());
        }

        if self.constitution <= 0 {
            errors.push("Constitution must be greater than 0".to_string());
        }

        if self.intelligence <= 0 {
            errors.push("Intelligence must be greater than 0".to_string());
        }

        if self.wisdom <= 0 {
            errors.push("Wisdom must be greater than 0".to_string());
        }

        if self.weapons.is_null() {
            errors.push("Weapons must be provided".to_string());
        }

        if self.armor.is_null() {
            errors.push("Armor must be provided".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

pub async fn create_enemy(
    State(state): State<AppState>,
    Path(world_code): Path<String>,
    Json(enemy): Json<EnemyDefinition>,
) -> ApiResponse<Enemy> {
    // todo: More information as to what is wrong with the item definition
    if let Err(errors) = enemy.is_valid() {
        return ApiResponse::BadRequest(errors);
    }

    let pool = state.db_pool.clone();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::Error("Failed to get connection".to_string()),
    };

    let world = match WorldRepository::find_by_code(&mut conn, &world_code) {
        Ok(world) => world,
        Err(_) => return ApiResponse::NotFound("World not found".to_string()),
    };

    let data_enemy = NewEnemy {
        world_id: world.id,
        code: enemy.code.clone(),
        name: enemy.name.clone(),
        description: enemy.description.clone(),
        class: enemy.class.clone(),
        level: enemy.level,
        hit_points: enemy.hit_points,
        stamina: enemy.stamina,
        strength: enemy.strength,
        dexterity: enemy.dexterity,
        constitution: enemy.constitution,
        intelligence: enemy.intelligence,
        wisdom: enemy.wisdom,
        weapons: enemy.weapons.clone(),
        armor: enemy.armor.clone(),
    };

    match EnemyRepository::create(&mut conn, &data_enemy) {
        Ok(enemy) => ApiResponse::Created(Payload { data: enemy }),
        Err(err) => ApiResponse::Error(format!("Failed to create item: {}", err).to_string()),
    }
}

pub async fn create_or_update_enemy(
    State(state): State<AppState>,
    Path((world_code, enemy_code)): Path<(String, String)>,
    Json(enemy): Json<EnemyDefinition>,
) -> ApiResponse<Enemy> {
    // todo: More information as to what is wrong with the item definition
    if let Err(errors) = enemy.is_valid() {
        return ApiResponse::BadRequest(errors);
    }

    let pool = state.db_pool.clone();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::Error("Failed to get connection".to_string()),
    };

    let world = match WorldRepository::find_by_code(&mut conn, &world_code) {
        Ok(world) => world,
        Err(_) => return ApiResponse::NotFound("World not found".to_string()),
    };

    let found_enemy = EnemyRepository::find_by_code(&mut conn, world.id, &enemy_code);
    let found_enemy_id = found_enemy.as_ref().map(|i| i.id).unwrap_or(0);

    let data_enemy = Enemy {
        id: found_enemy_id,
        world_id: world.id,
        code: enemy.code.clone(),
        name: enemy.name.clone(),
        description: enemy.description.clone(),
        class: enemy.class.clone(),
        level: enemy.level,
        hit_points: enemy.hit_points,
        stamina: enemy.stamina,
        strength: enemy.strength,
        dexterity: enemy.dexterity,
        constitution: enemy.constitution,
        intelligence: enemy.intelligence,
        wisdom: enemy.wisdom,
        weapons: enemy.weapons.clone(),
        armor: enemy.armor.clone(),
        created_at: enemy.created_at.unwrap_or(chrono::Utc::now().naive_utc()),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    // if the item is found, compare the signatures
    // if it is the same, return NotChanged
    // if it is not found or different, then just create or update the item.
    if let Ok(ref found_enemy) = found_enemy {
        if data_enemy.signature() == found_enemy.signature() {
            return ApiResponse::NotChanged;
        }
    }

    match EnemyRepository::create_or_update(&mut conn, &data_enemy) {
        Ok(enemy) => {
            if data_enemy.id == 0 {
                ApiResponse::Created(Payload { data: enemy })
            } else {
                ApiResponse::JsonData(Payload { data: enemy })
            }
        }
        Err(err) => ApiResponse::Error(format!("Failed to create enemy: {}", err).to_string()),
    }
}

pub async fn get_enemy_by_code(
    State(state): State<AppState>,
    axum::extract::Path((world_code, enemy_code)): axum::extract::Path<(String, String)>,
) -> ApiResponse<Enemy> {
    let pool = state.db_pool.clone();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::Error("Failed to get connection".to_string()),
    };

    let world = match WorldRepository::find_by_code(&mut conn, &world_code) {
        Ok(world) => world,
        Err(_) => return ApiResponse::NotFound("World not found".to_string()),
    };

    match EnemyRepository::find_by_code(&mut conn, world.id, &enemy_code) {
        Ok(enemy) => ApiResponse::JsonData(Payload { data: enemy }),
        Err(_) => ApiResponse::NotFound("Enemy not found".to_string()),
    }
}
