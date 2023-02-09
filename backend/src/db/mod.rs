use crate::error::Result;
use diesel::r2d2::ConnectionManager;
use diesel::{prelude::*, r2d2};
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
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

// We embed the migrations into the binary at compile time.
// This allows us to run the migrations without having to worry about the filesystem.
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

/// This function will run the database migrations, creating the tables if they do not exist.
/// This function should be called before the server starts.
///
/// # Errors
///
/// This function will panic if the migrations fail for any reason.
/// This is intentional, as the application cannot function without the database tables being created.
pub fn run_migrations(conn: &DBPool) -> Result<()> {
    log::info!("Running migrations");
    // run the migrations
    conn.get()
        .unwrap()
        .run_pending_migrations(MIGRATIONS)
        .unwrap();

    Ok(())
}
