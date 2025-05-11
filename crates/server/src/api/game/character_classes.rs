use std::error::Error;

use crate::api::{ApiResponse, Payload};
use crate::app_state::AppState;
use crate::game::{CharacterClass, CharacterClassFeature, World};
use crate::{CharacterClassFeatureRepository, CharacterClassRepository, WorldRepository};
use axum::Json;
use diesel::{Connection, PgConnection};

use protocol::types::Valid;
use protocol::{
    CharacterClass as ProtocolCharacterClass,
    CharacterClassFeature as ProtocolCharacterClassFeature, TypeSignature,
};

use axum::extract::{Path, State};

pub async fn get_character_class_by_code(
    State(state): State<AppState>,
    Path((world_code, class_code)): Path<(String, String)>,
) -> ApiResponse<ProtocolCharacterClass> {
    let pool = state.db_pool.clone();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::Error("Failed to get connection".to_string()),
    };

    let world = match WorldRepository::find_by_code(&mut conn, &world_code) {
        Ok(world) => world,
        Err(_) => return ApiResponse::NotFound("World not found".to_string()),
    };

    match get_character_class_and_features(&mut conn, &world, class_code.as_str()) {
        Ok(saved_class) => {
            if saved_class.is_none() {
                return ApiResponse::NotFound("character class not found".to_string());
            }
            ApiResponse::JsonData(Payload {
                data: saved_class.unwrap(),
            })
        }
        Err(e) => {
            eprintln!("Failed to get character class: {}", e);
            ApiResponse::Error("Failed to get character class".to_string())
        }
    }
}

pub async fn create_or_update_character_class(
    State(state): State<AppState>,
    Path((world_code, class_code)): Path<(String, String)>,
    Json(character_class): Json<ProtocolCharacterClass>,
) -> ApiResponse<ProtocolCharacterClass> {
    if let Err(errors) = character_class.validate() {
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

    let changes =
        match compute_character_class_changes(&mut conn, &world, &class_code, &character_class)
            .await
        {
            Ok(changes) => changes,
            Err(e) => {
                eprintln!("Failed to compute character class changes: {}", e);
                return ApiResponse::Error("Failed to compute character class changes".to_string());
            }
        };

    if !changes.has_changed() {
        return ApiResponse::NotChanged;
    }

    let mut entity_character_class = protocol_character_class_to_entity(&character_class);

    let txn_result = conn.transaction(|txn| {
        let changes = &changes;
        let class_changes = changes.class_changes.clone();
        let feature_changes = changes.feature_changes.clone();
        let removed_features = changes.removed_features.clone();

        let saved_class = match class_changes {
            Some(_) => {
                if changes.existing_class.is_some() {
                    let existing_class = changes.existing_class.clone().unwrap();
                    entity_character_class.id = existing_class.id.unwrap_or(0);
                }
                entity_character_class.world_id = world.id;

                println!("Saving character class: {:?}", entity_character_class);
                match CharacterClassRepository::create_or_update(txn, &entity_character_class) {
                    Ok(saved_class) => saved_class,
                    Err(e) => return Err(e),
                }
            }
            None => entity_character_class.clone(),
        };

        if feature_changes.is_none() && removed_features.is_none() {
            return diesel::QueryResult::Ok(());
        }

        if let Some(feature_changes) = feature_changes {
            for feature in feature_changes {
                let mut new_feature = protocol_character_class_feature_to_entity(&feature);
                new_feature.class_id = saved_class.id;
                // for now delete the feature and re-add it
                match CharacterClassFeatureRepository::delete_by_code(
                    txn,
                    saved_class.id,
                    &new_feature.code,
                ) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                match CharacterClassFeatureRepository::create_or_update_feature(txn, &new_feature) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }
        }

        if let Some(removed_features) = removed_features {
            for feature_code in removed_features {
                match CharacterClassFeatureRepository::delete_by_code(
                    txn,
                    saved_class.id,
                    &feature_code,
                ) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }
        }
        diesel::QueryResult::Ok(())
    });

    match txn_result {
        Ok(_) => match get_character_class_and_features(&mut conn, &world, class_code.as_str()) {
            Ok(saved_class) => {
                if saved_class.is_none() {
                    eprintln!("Severe error: character class not found after saving");
                    return ApiResponse::Error("character class not found".to_string());
                }
                ApiResponse::JsonData(Payload {
                    data: saved_class.unwrap(),
                })
            }
            Err(e) => {
                eprintln!("Failed to get character class: {}", e);
                ApiResponse::Error("Failed to get character class".to_string())
            }
        },
        Err(e) => {
            eprintln!("Failed to save character class: {}", e);
            ApiResponse::Error("Failed to save character class".to_string())
        }
    }
}

fn get_character_class_and_features(
    conn: &mut PgConnection,
    world: &World,
    class_code: &str,
) -> Result<Option<ProtocolCharacterClass>, Box<dyn Error>> {
    let entity_character_class =
        match CharacterClassRepository::find_character_class_by_code(conn, world.id, class_code) {
            Ok(character) => Some(character),
            Err(diesel::result::Error::NotFound) => None,
            Err(e) => return Err(Box::new(e)),
        };

    if entity_character_class.is_none() {
        return Ok(None);
    }

    let entity_class = match entity_character_class {
        Some(ref character) => character,
        None => return Ok(None),
    };

    let entity_features =
        match CharacterClassFeatureRepository::find_by_class(conn, entity_class.id) {
            Ok(features) => features,
            Err(e) => return Err(Box::new(e)),
        };

    let mut protocol_character_class = ProtocolCharacterClass {
        id: Some(entity_class.id),
        code: Some(entity_class.code.clone()),
        world_id: Some(entity_class.world_id),
        name: entity_class.name.clone(),
        description: entity_class.description.clone(),
        stamina_expression: entity_class.stamina_expression.clone(),
        hit_points: entity_class.hit_points,
        skillpoint_expression: entity_class.skillpoint_expression.clone(),
        proficiencies: Some(
            entity_class
                .proficiencies
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|v| v.as_str().unwrap_or("").to_string())
                .collect(),
        ),
        features: None,
    };

    entity_features.iter().for_each(|feature| {
        let protocol_feature = ProtocolCharacterClassFeature {
            level: feature.level,
            code: feature.code.clone(),
            name: feature.name.clone(),
            description: feature.description.clone(),
        };

        protocol_character_class
            .features
            .get_or_insert(vec![])
            .push(protocol_feature);
    });

    Ok(Some(protocol_character_class))
}

fn protocol_character_class_to_entity(character: &ProtocolCharacterClass) -> CharacterClass {
    CharacterClass {
        id: character.id.unwrap_or(0),
        world_id: character.world_id.unwrap_or(0),
        code: character.code.clone().unwrap_or("".to_string()),
        name: character.name.clone(),
        description: character.description.clone(),
        hit_points: character.hit_points,
        stamina_expression: character.stamina_expression.clone(),
        skillpoint_expression: character.skillpoint_expression.clone(),
        proficiencies: serde_json::to_value(&character.proficiencies).unwrap_or_default(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    }
}

fn protocol_character_class_feature_to_entity(
    feature: &ProtocolCharacterClassFeature,
) -> CharacterClassFeature {
    CharacterClassFeature {
        id: 0,
        class_id: 0,
        level: feature.level,
        code: feature.code.clone(),
        name: feature.name.clone(),
        description: feature.description.clone(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    }
}

struct CharacterClassDelta {
    existing_class: Option<ProtocolCharacterClass>,
    class_changes: Option<ProtocolCharacterClass>,
    feature_changes: Option<Vec<ProtocolCharacterClassFeature>>,
    removed_features: Option<Vec<String>>,
}

impl CharacterClassDelta {
    fn has_changed(&self) -> bool {
        self.class_changes.is_some()
            || self.feature_changes.is_some()
            || self.removed_features.is_some()
    }
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
    world: &World,
    class_code: &str,
    candidate: &ProtocolCharacterClass,
) -> Result<CharacterClassDelta, Box<dyn Error>> {
    let existing_class = match get_character_class_and_features(conn, world, class_code) {
        Ok(class_) => class_,
        Err(e) => return Err(e),
    };

    let mut class_changes = CharacterClassDelta {
        existing_class: None,
        class_changes: None,
        feature_changes: None,
        removed_features: None,
    };

    if existing_class.is_none() {
        class_changes.class_changes = Some(candidate.clone());
        class_changes.feature_changes = candidate.features.clone();
    }

    if existing_class.is_some() {
        if existing_class.clone().unwrap().signature() == candidate.signature() {
            return Ok(class_changes);
        } else {
            println!(
                "Existing class: {:?}",
                pretty_print_vec(existing_class.clone().unwrap().signature().as_ref())
            );
            println!(
                "Candidate class: {:?}",
                pretty_print_vec(candidate.signature().as_ref())
            );
            println!(
                "Name: (existing) {:?} (candidate) {:?}",
                existing_class.clone().unwrap().name,
                candidate.name
            );
            println!("Existing class: {:?}", existing_class);
            println!("Candidate class: {:?}", candidate);
        }

        class_changes.existing_class = existing_class.clone();
        class_changes.class_changes = Some(candidate.clone());
        class_changes.feature_changes = candidate.features.clone();
    }

    Ok(class_changes)
}

fn pretty_print_vec(vec: &Vec<u8>) -> String {
    let mut result = String::new();
    for byte in vec {
        result.push_str(&format!("{:02x}", byte));
    }
    result
}
