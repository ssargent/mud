use crate::{
    api::{ApiResponse, Payload},
    app_state::AppState,
    db::{CharacterRepository, WorldRepository},
};
use axum::extract::State;
use protocol::types::Character;

pub async fn player_get_character(
    State(state): State<AppState>,
    axum::extract::Path((world_code, character_id)): axum::extract::Path<(String, i64)>,
) -> ApiResponse<Character> {
    let pool = state.db_pool.clone();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::Error("Failed to get connection".to_string()),
    };

    let world = match WorldRepository::find_by_code(&mut conn, &world_code) {
        Ok(world) => world,
        Err(_) => return ApiResponse::NotFound("World not found".to_string()),
    };

    match CharacterRepository::find_by_id(&mut conn, world.id, character_id) {
        Ok(character) => ApiResponse::JsonData(Payload {
            data: character.as_protocol_character(),
        }),
        Err(_) => ApiResponse::NotFound("Character not found".to_string()),
    }
}
