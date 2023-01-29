use crate::{
    db::{
        auth::{
            login::{authenticate_user, LoginUser},
            register::{register_user, RegisterUser},
        },
        DBPool,
    },
    error::Result,
};
use actix_session::Session;
use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder, Scope,
};

/// This function returns a Scope that contains all the routes for the auth module.
pub fn auth_scope() -> Scope {
    Scope::new("/auth")
        .service(login)
        .service(logout)
        .service(register)
}

/// This function is used to log a user in given a username and password.
/// It will return an error if the username or password is incorrect.
#[post("/login")]
pub async fn login(
    session: Session,
    pool: Data<DBPool>,
    user: web::Json<LoginUser>,
) -> Result<impl Responder> {
    let user = authenticate_user(user.into_inner(), &mut pool.get().unwrap())?;

    // Log the user in
    session.insert("email", user.email)?;

    Ok(HttpResponse::Ok())
}

/// This function is used to register a new user.
#[post("/register")]
pub async fn register(
    session: Session,
    pool: Data<DBPool>,
    user: web::Json<RegisterUser>,
) -> Result<impl Responder> {
    let user = register_user(user.into_inner(), &mut pool.get().unwrap())?;

    // Log the user in
    session.insert("email", user.email)?;

    Ok(HttpResponse::Ok())
}

/// This function is used to log a user out.
#[post("/logout")]
pub async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok()
}
