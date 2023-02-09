use crate::{db::run_migrations, error::Result};
use actix_cors::Cors;
use actix_files::Files;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
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

    // check for the docker environment variable
    let docker = std::env::var("DOCKER").is_ok();

    let frontend_build_dir = if docker {
        // if we are running in docker, we need to serve the frontend from the build
        "./frontend/build"
    } else {
        // if we are not running in docker, we can serve the frontend from its source directory relative to the backend
        "../frontend/build"
    };

    // check the startup configuration
    // can only be dev or prod
    let env = std::env::var("ENV").unwrap_or_else(|_| "dev".to_string());

    // some startup configuration
    // 0.0.0.0 exposes the server to the local network
    // so we can expose it to host machine from docker
    let host = "0.0.0.0";
    let port = 8000;

    let db_pool = db::establish_connection()?;

    // run the migrations
    run_migrations(&db_pool)?;

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
        let mut session =
            SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                // allow cookies to be sent over http
                .cookie_secure(false)
                // encrypt the cookie
                .cookie_content_security(actix_session::config::CookieContentSecurity::Private)
                // set the session ttl
                .session_lifecycle(
                    PersistentSession::default()
                        .session_ttl(actix_web::cookie::time::Duration::minutes(10)),
                );

        if env == "dev" {
            // allow cookies to work over localhost
            session = session.cookie_same_site(SameSite::Lax);
        } else {
            // allow cookies to be sent to other domains
            // im not sure why lax doesnt work here
            session = session.cookie_same_site(SameSite::Strict);
        }

        let session = session.build();

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
            // we have to put this at the end so that it doesnt override the api routes
            .service(
                Files::new("/", frontend_build_dir)
                    .index_file("index.html")
                    .default_handler(
                        actix_files::NamedFile::open(format!("{}/index.html", frontend_build_dir))
                            .unwrap(),
                    ),
            )
    })
    .bind((host, port))?
    .run()
    .await?;

    Ok(())
}
