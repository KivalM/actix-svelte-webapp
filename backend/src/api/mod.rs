use actix_web::Scope;

pub mod auth;

/// This is the main scope for the api module.
pub fn api_scope() -> Scope {
    Scope::new("/api").service(auth::auth_scope())
}
