use crate::db::models::system::{NewUser, User};
use crate::db::system_schema::system::users::dsl::*;
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
}
