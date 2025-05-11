use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::sync::Arc;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<DbPool>,
}

impl AppState {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create database connection pool.");

        Self {
            db_pool: Arc::new(pool),
        }
    }
}
