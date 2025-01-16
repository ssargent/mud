use crate::db::game_schema::game::character_classes::dsl::*;
use crate::db::models::game::{CharacterClass, NewCharacterClass};
use diesel::prelude::*;

pub struct CharacterClassRepository;

impl CharacterClassRepository {
    pub fn find_character_class_by_code(
        conn: &mut PgConnection,
        world_id_value: i64,
        character_class_code: &str,
    ) -> QueryResult<CharacterClass> {
        character_classes
            .filter(world_id.eq(world_id_value))
            .filter(code.eq(character_class_code))
            .select(CharacterClass::as_select())
            .first(conn)
    }

    pub fn create(
        conn: &mut PgConnection,
        new_character_class: &NewCharacterClass,
    ) -> QueryResult<CharacterClass> {
        diesel::insert_into(character_classes)
            .values(new_character_class)
            .returning(CharacterClass::as_select())
            .get_result(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        character_class: &CharacterClass,
    ) -> QueryResult<CharacterClass> {
        diesel::update(character_classes)
            .filter(id.eq(&character_class.id))
            .set((
                name.eq(&character_class.name),
                description.eq(&character_class.description),
                updated_at.eq(&character_class.updated_at),
                stamina_expression.eq(&character_class.stamina_expression),
                skillpoint_expression.eq(&character_class.skillpoint_expression),
                proficiencies.eq(&character_class.proficiencies),
            ))
            .returning(CharacterClass::as_select())
            .get_result(conn)
    }

    pub fn create_or_update(
        conn: &mut PgConnection,
        character_class: &CharacterClass,
    ) -> QueryResult<CharacterClass> {
        if character_class.id == 0 {
            CharacterClassRepository::create(conn, &character_class.as_new_character_class())
        } else {
            CharacterClassRepository::update(conn, character_class)
        }
    }
}
