use crate::api::{ApiResponse, Payload};
use crate::app_state::AppState;
use crate::game::Capability;
use crate::{CapabilitiesRepository, WorldRepository};
use axum::Json;
use chrono::NaiveDateTime;

use protocol::{Capability as ProtocolCapability, TypeSignature};

use axum::extract::{Path, State};

pub async fn get_capabilties_by_type(
    State(state): State<AppState>,
    Path((world_code, capability_type_param)): Path<(String, String)>,
) -> ApiResponse<Vec<ProtocolCapability>> {
    let pool = state.db_pool.clone();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::Error("Failed to get connection".to_string()),
    };

    let world = match WorldRepository::find_by_code(&mut conn, &world_code) {
        Ok(world) => world,
        Err(_) => return ApiResponse::NotFound("World not found".to_string()),
    };

    let capabilities =
        match CapabilitiesRepository::find_by_type(&mut conn, world.id, &capability_type_param) {
            Ok(capabilities) => capabilities,
            Err(_) => return ApiResponse::Error("Failed to get capabilities".to_string()),
        };

    let protocol_capabilities: Vec<ProtocolCapability> =
        capabilities.iter().map(to_protocol_capability).collect();

    ApiResponse::JsonData(Payload {
        data: protocol_capabilities,
    })
}

pub async fn create_capability(
    State(state): State<AppState>,
    Path((world_code, capability_type_param, capability_code)): Path<(String, String, String)>,
    Json(capability): Json<ProtocolCapability>,
) -> ApiResponse<ProtocolCapability> {
    if capability_type_param != capability.capability_type {
        return ApiResponse::Error("Capability type does not match".to_string());
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

    let found_capability =
        match CapabilitiesRepository::find_by_code(&mut conn, world.id, &capability_code) {
            Ok(capability) => Some(capability),
            Err(_) => None,
        };

    // check to see if there have been changes, if so return not changed
    if found_capability.is_some()
        && found_capability
            .as_ref()
            .map(to_protocol_capability)
            .map(|c| c.signature())
            == Some(capability.signature())
    {
        return ApiResponse::NotChanged;
    }

    let mut entity_capability = from_protocol_capability(&capability);
    entity_capability.world_id = world.id;
    entity_capability.id = found_capability.map(|c| c.id).unwrap_or(0);

    let created_capability =
        match CapabilitiesRepository::create_or_update(&mut conn, &entity_capability) {
            Ok(capability) => capability,
            Err(err) => {
                return ApiResponse::Error(
                    format!("Failed to create capability: {}", err).to_string(),
                )
            }
        };

    ApiResponse::JsonData(Payload {
        data: to_protocol_capability(&created_capability),
    })
}

fn to_protocol_capability(capability: &Capability) -> ProtocolCapability {
    ProtocolCapability {
        id: Some(capability.id),
        world_id: Some(capability.world_id),
        parent_id: capability.parent_id,
        capability_type: capability.capability_type.clone(),
        code: capability.code.clone(),
        name: capability.name.clone(),
        description: capability.description.clone(),
        requirements: capability.requirements.clone(),
        actions: capability.actions.clone(),
        access_requirements: capability.access_requirements.clone(),
        gameplay_definition: capability.gameplay_definition.clone(),
        tags: capability.tags.clone(),
    }
}

fn from_protocol_capability(capability: &ProtocolCapability) -> Capability {
    Capability {
        id: capability.id.unwrap_or(0),
        world_id: capability.world_id.unwrap_or(0),
        parent_id: capability.parent_id,
        capability_type: capability.capability_type.clone(),
        code: capability.code.clone(),
        name: capability.name.clone(),
        description: capability.description.clone(),
        requirements: capability.requirements.clone(),
        actions: capability.actions.clone(),
        access_requirements: capability.access_requirements.clone(),
        gameplay_definition: capability.gameplay_definition.clone(),
        tags: capability.tags.clone(),
        created_at: NaiveDateTime::default(),
        updated_at: NaiveDateTime::default(),
    }
}
