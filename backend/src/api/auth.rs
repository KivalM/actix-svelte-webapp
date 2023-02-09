use crate::{
    db::{
        auth::{
            get_all_users, get_user_by_email,
            login::{authenticate_user, LoginUser},
            register::{register_user, RegisterUser},
        },
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
        .service(get_users)
}

/// This function is used to log a user in given a username and password.
/// It will return an error if the username or password is incorrect.
#[post("/login")]
pub async fn login(
    session: Session,
    pool: Data<DBPool>,
    user: web::Json<LoginUser>,
) -> Result<impl Responder> {
    let user = web::block(move || authenticate_user(user.into_inner(), &mut pool.get().unwrap()))
        .await??;

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
    let user =
        web::block(move || register_user(user.into_inner(), &mut pool.get().unwrap())).await??;

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
    let email = session.get::<String>("email")?;

    match email {
        Some(email) => {
            let user = web::block(move || {
                let conn = &mut pool.get().unwrap();
                get_user_by_email(email, conn)
            })
            .await??;
            Ok(HttpResponse::Ok().json(user))
        }
        None => Ok(HttpResponse::Unauthorized().into()),
    }
}

#[get("/users")]
pub async fn get_users(pool: Data<DBPool>) -> Result<impl Responder> {
    let users = web::block(move || {
        let conn = &mut pool.get().unwrap();
        get_all_users(conn)
    })
    .await??;

    Ok(HttpResponse::Ok().json(users))
}
