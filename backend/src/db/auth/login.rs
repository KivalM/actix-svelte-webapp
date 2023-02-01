use crate::error::CrateError::*;
use crate::{db::models::User, error::Result, schema::users};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl};
use diesel::{OptionalExtension, QueryDsl};

use serde::Deserialize;

/// This struct will be used to deserialize the request body for the login route.
#[derive(Deserialize, Debug)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

/// This function will authenticate a user by their email and password.
///
/// It will return a user if the email and password are correct.
pub fn authenticate_user(user: LoginUser, conn: &mut PgConnection) -> Result<User> {
    let table_user = users::table
        .filter(users::email.eq(user.email))
        .first::<User>(conn)
        .optional()?;

    // check if the user exists
    let table_user = match table_user {
        Some(user) => user,
        None => return Err(UserNotFound),
    };

    // validate password with argon2
    let parsed_hash = PasswordHash::new(&table_user.password_hash).unwrap();
    let matches = Argon2::default()
        .verify_password(user.password.as_bytes(), &parsed_hash)
        .is_ok();

    if matches {
        Ok(table_user)
    } else {
        Err(InvalidCredentials)
    }
}
