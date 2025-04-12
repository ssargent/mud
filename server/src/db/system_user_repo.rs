use super::system::{ActiveUserRole, NewUser, User};
use crate::db::system_schema::system::users::dsl::*;
use crate::db::system_schema::system::{roles, user_roles};
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

    pub fn get_active_user_roles(
        conn: &mut PgConnection,
        user_id_val: i64,
    ) -> QueryResult<Vec<ActiveUserRole>> {
        use roles::dsl as r;
        use user_roles::dsl as ur;

        r::roles
            .inner_join(ur::user_roles)
            .filter(ur::user_id.eq(user_id_val))
            .filter(ur::start_date.le(diesel::dsl::now))
            .filter(ur::end_date.is_null().or(ur::end_date.ge(diesel::dsl::now)))
            .select((r::name, ur::is_read_only))
            .load(conn)
    }
}
