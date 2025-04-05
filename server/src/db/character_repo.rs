use crate::db::models::player::Character;
use crate::db::player_schema::player::characters::dsl::*;
use diesel::prelude::*;

pub struct CharacterRepository {}

impl CharacterRepository {
    pub fn find_by_id(
        conn: &mut PgConnection,
        character_world_id: i64,
        character_id: i64,
    ) -> QueryResult<Character> {
        characters
            .filter(world_id.eq(character_world_id))
            .filter(id.eq(character_id))
            .select(Character::as_select())
            .first(conn)
    }
}
