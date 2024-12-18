use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Queryable;
use diesel::Selectable;
use uuid::Uuid;

#[derive(Queryable, QueryableByName, Selectable, Identifiable, Debug, Clone)]
#[diesel(table_name = crate::system_schema::system::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub full_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
