use crate::error::Result;
use actix_cors::Cors;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    middleware::{Compress, Logger},
    web::Data,
    App, HttpServer,
};
use log::info;

pub mod api;
pub mod db;
pub mod error;
pub mod schema;

#[actix_web::main]
async fn main() -> Result<()> {
    // start the logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    // some startup configuration
    let port = 8000;
    let host = "127.0.0.1";
    let db_pool = db::establish_connection()?;

    // startup logging
    info!("Starting up the server");
    info!("Listening on http://{}:{}", host, port);

    // randomly generate the secret for actix cookie session
    // This would be stored in an environment variable in a production application
    // but for this example we will just generate a new one each time the server starts.
    let secret_key = Key::generate();

    HttpServer::new(move || {
        // configure cors
        let cors = Cors::permissive();

        // configure actix session
        let session = SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
            // allow cookies to be sent over http
            .cookie_secure(false)
            // encrypt the cookie
            .cookie_content_security(actix_session::config::CookieContentSecurity::Private)
            // set the session ttl
            .session_lifecycle(
                PersistentSession::default()
                    .session_ttl(actix_web::cookie::time::Duration::minutes(10)),
            )
            .build();

        App::new()
            .app_data(Data::new(db_pool.clone()))
            .wrap(cors)
            // wrap logger middleware
            .wrap(Logger::default())
            // wrap session middleware
            .wrap(session)
            // wrap compression middleware
            .wrap(Compress::default())
            // load the full api scope
            .service(api::api_scope())
    })
    .bind((host, port))?
    .run()
    .await?;

    Ok(())
}
