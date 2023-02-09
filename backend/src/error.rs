use diesel::r2d2;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, CrateError>;

#[derive(Error, Debug)]
pub enum CrateError {
    #[error("Failed to start server.")]
    ServerStartError(#[from] std::io::Error),
    #[error("Failed to run Query.")]
    DieselError(#[from] diesel::result::Error),

    // r2d2 errors
    #[error("Failed to get connection from pool.")]
    R2D2Error(#[from] r2d2::Error),

    // auth flow errors
    #[error("Username or password is incorrect.")]
    InvalidCredentials,
    #[error("Username is taken.")]
    UsernameIsTaken,
    #[error("There is already an account with that email.")]
    EmailIsTaken,
    #[error("No user with that details.")]
    UserNotFound,
    // session errors
    #[error("Could not insert key into session.")]
    SessionInsertError(#[from] actix_session::SessionInsertError),
    #[error("Could not get key from session.")]
    SessionExtractError(#[from] actix_session::SessionGetError),

    // web::block errors
    #[error("Failed to run web::block.")]
    WebBlockError(#[from] actix_web::error::BlockingError),
}

/// ResponseError trait is used to convert errors into http responses.
/// We use this to catch errors in the middleware and convert them into http responses.
impl actix_web::ResponseError for CrateError {
    fn error_response(&self) -> actix_web::HttpResponse {
        // use the error's Display implementation to get a string representation of the error
        let message = self.to_string();

        // create an http response with the error message
        actix_web::HttpResponse::InternalServerError().body(message)
    }
}
