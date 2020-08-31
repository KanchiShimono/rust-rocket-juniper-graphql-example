pub mod models;
pub mod repository;
pub mod schema;

#[database("postgres_db")]
pub struct Db(diesel::PgConnection);
