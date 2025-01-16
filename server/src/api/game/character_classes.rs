use std::error::{self, Error};

use axum::Json;
use diesel::{Connection, PgConnection};
use protocol::TypeSignature;

use crate::api::{ApiResponse, Payload};
use crate::app_state::AppState;
use crate::game::{
    CharacterClass, CharacterClassFeature, NewCharacterClass, NewCharacterClassFeature,
};
use crate::{CharacterClassFeatureRepository, CharacterClassRepository, WorldRepository};

use axum::extract::{Path, State};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterClassDefinition {
    pub id: Option<i64>,
    pub world_id: Option<i64>,
    pub code: Option<String>,
    pub name: String,
    pub description: String,
    pub hit_points: i32,
    pub stamina_expression: String,
    pub skillpoint_expression: String,
    pub proficiencies: Option<Vec<String>>,
    pub features: Option<Vec<CharacterClassFeatureDefinition>>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl CharacterClassDefinition {
    pub fn is_valid(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.code.is_none() {
            errors.push("Code is required".to_string());
        }

        if self.name.is_empty() {
            errors.push("Name is required".to_string());
        }

        if self.description.is_empty() {
            errors.push("Description is required".to_string());
        }

        if self.hit_points <= 0 {
            errors.push("Hit points must be greater than 0".to_string());
        }

        if self.stamina_expression.is_empty() {
            errors.push("Stamina expression is required".to_string());
        }

        if self.skillpoint_expression.is_empty() {
            errors.push("Skillpoint expression is required".to_string());
        }

        if self.proficiencies.is_none() {
            errors.push("Proficiencies are required".to_string());
        }

        if let Some(proficiencies) = &self.proficiencies {
            for proficiency in proficiencies {
                if proficiency.is_empty() {
                    errors.push("Proficiency cannot be empty".to_string());
                }
            }
        }

        if self.features.is_none() {
            errors.push("Features are required".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn to_entity(&self) -> CharacterClass {
        CharacterClass {
            id: self.id.unwrap_or_default(),
            world_id: self.world_id.unwrap_or_default(),
            code: self.code.clone().unwrap_or_default(),
            name: self.name.clone(),
            description: self.description.clone(),
            hit_points: self.hit_points,
            stamina_expression: self.stamina_expression.clone(),
            skillpoint_expression: self.skillpoint_expression.clone(),
            proficiencies: serde_json::to_value(self.proficiencies.clone()).unwrap(),
            created_at: self
                .created_at
                .unwrap_or_else(|| chrono::Utc::now().naive_utc()),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterClassFeatureDefinition {
    pub level: i32,
    pub code: String,
    pub name: String,
    pub description: String,
}

impl TypeSignature for CharacterClassFeatureDefinition {
    fn signature(&self) -> Vec<u8> {
        let mut signature = Vec::new();
        signature.extend_from_slice(&self.level.to_be_bytes());
        signature.extend_from_slice(self.code.as_bytes());
        signature.extend_from_slice(self.name.as_bytes());
        signature.extend_from_slice(self.description.as_bytes());

        Self::as_hashed(signature)
    }
}

pub async fn create_or_update_character_class(
    State(state): State<AppState>,
    Path((world_code, class_code)): Path<(String, String)>,
    Json(character_class): Json<CharacterClassDefinition>,
) -> ApiResponse<CharacterClass> {
    if let Err(errors) = character_class.is_valid() {
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

    let changes = compute_character_class_changes(&mut conn, &character_class).await;

    let entity_character_class = character_class.to_entity();

    let txn_result = conn.transaction(|mut txn| {
        // this needs to examine changes and update the character class and features accordingly
        let saved_class =
            match CharacterClassRepository::create_or_update(&mut txn, &entity_character_class) {
                Ok(saved_class) => saved_class,
                Err(e) => return diesel::QueryResult::Err(e),
            };

        diesel::QueryResult::Ok(saved_class)
    });

    match txn_result {
        Ok(saved_class) => ApiResponse::JsonData(Payload { data: saved_class }),
        Err(e) => {
            eprintln!("Failed to save character class: {}", e);
            ApiResponse::Error("Failed to save character class".to_string())
        }
    }
}

struct CharacterClassDelta {
    class_changes: Option<CharacterClassDefinition>,
    feature_changes: Option<Vec<CharacterClassFeatureDefinition>>,
    removed_features: Option<Vec<String>>,
}

/**
Compute the changes between the candidate character class definition and the existing character class definition.
 This function will return a `CharacterClassDelta` struct that contains the changes between the two character class definitions.
 The `class_changes` field will contain the changes to the character class definition itself.
 The `feature_changes` field will contain the changes to the character class features.
 The `removed_features` field will contain the features that have been removed from the candidate character class definition.
*/
async fn compute_character_class_changes(
    conn: &mut PgConnection,
    candidate: &CharacterClassDefinition,
) -> Option<CharacterClassDelta> {
    todo!()
}
