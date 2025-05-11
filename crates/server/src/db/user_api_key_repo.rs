use crate::db::system_schema::system::user_api_keys::dsl::*;
use diesel::prelude::*;

use super::system::{NewUserApiKey, UserApiKey};

pub struct UserApiKeyRepository;

impl UserApiKeyRepository {
    pub fn find_by_id(conn: &mut PgConnection, user_api_key_id: i64) -> QueryResult<UserApiKey> {
        user_api_keys
            .filter(id.eq(user_api_key_id))
            .select(UserApiKey::as_select())
            .first(conn)
    }

    pub fn find_by_user_id(
        conn: &mut PgConnection,
        user_id_value: i64,
    ) -> QueryResult<Vec<UserApiKey>> {
        user_api_keys
            .filter(user_id.eq(user_id_value))
            .select(UserApiKey::as_select())
            .load::<UserApiKey>(conn)
    }

    pub fn create(
        conn: &mut PgConnection,
        new_user_api_key: &NewUserApiKey,
    ) -> QueryResult<UserApiKey> {
        diesel::insert_into(user_api_keys)
            .values(new_user_api_key)
            .returning(UserApiKey::as_select())
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, user_api_key: &UserApiKey) -> QueryResult<UserApiKey> {
        diesel::update(user_api_keys)
            .filter(id.eq(&user_api_key.id))
            .set((
                expiration.eq(&user_api_key.expiration),
                updated_at.eq(&user_api_key.updated_at),
                updated_by.eq(&user_api_key.updated_by),
            ))
            .returning(UserApiKey::as_select())
            .get_result(conn)
    }
}
