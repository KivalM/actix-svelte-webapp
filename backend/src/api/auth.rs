use crate::{
    db::{
        auth::{
            get_user_by_email,
            login::{authenticate_user, LoginUser},
            register::{register_user, RegisterUser},
        },
        models::DisplayableUser,
        DBPool,
    },
    error::Result,
};
use actix_session::Session;
use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder, Scope,
};

/// This function returns a Scope that contains all the routes for the auth module.
pub fn auth_scope() -> Scope {
    Scope::new("/auth")
        .service(login)
        .service(logout)
        .service(register)
        .service(get_user)
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
    session.renew();
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

#[post("/user")]
pub async fn get_user(session: Session, pool: Data<DBPool>) -> Result<impl Responder> {
    let conn = &mut pool.get().unwrap();
    let email = session.get::<String>("email")?;

    let user: DisplayableUser = match email {
        Some(email) => get_user_by_email(email, conn)?.into(),
        None => return Ok(HttpResponse::Unauthorized().into()),
    };

    Ok(HttpResponse::Ok().json(user))
}
