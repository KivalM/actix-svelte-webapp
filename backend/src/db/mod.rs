use crate::error::Result;
use diesel::r2d2::ConnectionManager;
use diesel::{prelude::*, r2d2};
use dotenvy::dotenv;

pub mod auth;
pub mod models;

/// This is a custom type alias for the connection pool.
pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// This function establishes a connection to the database.
///
/// # Errors
///
/// This function will panic if the connection to the database fails for any reason.
/// This is intentional, as the application cannot function without a database connection.
pub fn establish_connection() -> Result<DBPool> {
    // load the environment variables
    dotenv().ok();

    // get the connection string from the environment
    let conn_string = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create the connection pool
    let conn_manager = ConnectionManager::<PgConnection>::new(conn_string);
    let conn_pool = r2d2::Pool::builder()
        .build(conn_manager)
        .expect("Failed to create pool.");

    Ok(conn_pool)
}
