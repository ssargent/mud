use super::system::{ActiveUserRole, NewUser, User};
use crate::db::system_schema::system::users::dsl::*;
use crate::db::system_schema::system::{
    permissions, role_permissions, roles, user_api_keys, user_roles, users,
};
use diesel::prelude::*;

pub struct SystemUserRepository;

impl SystemUserRepository {
    pub fn create(conn: &mut PgConnection, new_user: &NewUser) -> QueryResult<User> {
        diesel::insert_into(users)
            .values(new_user)
            .returning(User::as_select())
            .get_result(conn)
    }

    pub fn get_by_email(conn: &mut PgConnection, email_value: &str) -> QueryResult<Option<User>> {
        users
            .filter(email.eq(email_value))
            .select(User::as_select())
            .first::<User>(conn)
            .optional()
    }

    pub fn get_by_id(conn: &mut PgConnection, user_id_value: i64) -> QueryResult<User> {
        users
            .filter(id.eq(user_id_value))
            .select(User::as_select())
            .first(conn)
    }

    pub fn get_user_permissions(
        conn: &mut PgConnection,
        user_id_val: i64,
    ) -> QueryResult<Vec<String>> {
        use permissions::dsl as p;
        use role_permissions::dsl as rp;
        use roles::dsl as r;
        use user_roles::dsl as ur;

        r::roles
            .inner_join(ur::user_roles)
            .inner_join(rp::role_permissions)
            .inner_join(p::permissions.on(rp::permission_id.eq(p::id)))
            .filter(ur::user_id.eq(user_id_val))
            .select(p::code)
            .load(conn)
    }

    pub fn get_user_by_api_key(
        conn: &mut PgConnection,
        api_key_value: &str,
    ) -> QueryResult<Option<User>> {
        use user_api_keys::dsl as uak;
        use users::dsl as u;

        u::users
            .inner_join(uak::user_api_keys)
            .filter(uak::api_key.eq(api_key_value))
            .select(User::as_select())
            .first::<User>(conn)
            .optional()
    }

    pub fn get_active_user_roles(
        conn: &mut PgConnection,
        user_id_val: i64,
    ) -> QueryResult<Vec<ActiveUserRole>> {
        use roles::dsl as r;
        use user_roles::dsl as ur;

        r::roles
            .inner_join(ur::user_roles)
            .filter(ur::user_id.eq(user_id_val))
            .filter(ur::start_date.le(diesel::dsl::now)) // Use `now` directly
            .filter(
                ur::end_date.is_null().or(ur::end_date.ge(diesel::dsl::now)), // Use `now` directly
            )
            .select((r::name, ur::is_read_only))
            .load(conn)
    }
}
