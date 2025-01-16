use crate::db::game_schema::game::character_class_features::dsl::*;
use crate::db::models::game::{CharacterClassFeature, NewCharacterClassFeature};
use diesel::prelude::*;

pub struct CharacterClassFeatureRepository;

impl CharacterClassFeatureRepository {
    pub fn create_feature(
        conn: &mut PgConnection,
        new_feature: &NewCharacterClassFeature,
    ) -> QueryResult<CharacterClassFeature> {
        diesel::insert_into(character_class_features)
            .values(new_feature)
            .returning(CharacterClassFeature::as_select())
            .get_result(conn)
    }

    pub fn update_feature(
        conn: &mut PgConnection,
        feature: &CharacterClassFeature,
    ) -> QueryResult<CharacterClassFeature> {
        diesel::update(character_class_features)
            .filter(id.eq(&feature.id))
            .set((
                name.eq(&feature.name),
                description.eq(&feature.description),
                updated_at.eq(&feature.updated_at),
            ))
            .returning(CharacterClassFeature::as_select())
            .get_result(conn)
    }

    pub fn create_or_update_feature(
        conn: &mut PgConnection,
        feature: &CharacterClassFeature,
    ) -> QueryResult<CharacterClassFeature> {
        if feature.id == 0 {
            CharacterClassFeatureRepository::create_feature(
                conn,
                &feature.as_new_character_class_feature(),
            )
        } else {
            CharacterClassFeatureRepository::update_feature(conn, feature)
        }
    }

    pub fn find_by_class(
        conn: &mut PgConnection,
        character_class_id: i64,
    ) -> QueryResult<Vec<CharacterClassFeature>> {
        character_class_features
            .filter(class_id.eq(character_class_id))
            .select(CharacterClassFeature::as_select())
            .load(conn)
    }

    pub fn find_feature_by_code(
        conn: &mut PgConnection,
        class_id_value: i64,
        feature_code: &str,
    ) -> QueryResult<CharacterClassFeature> {
        character_class_features
            .filter(class_id.eq(class_id_value))
            .filter(code.eq(feature_code))
            .select(CharacterClassFeature::as_select())
            .first(conn)
    }
}
