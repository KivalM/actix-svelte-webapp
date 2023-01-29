use crate::error::CrateError::*;
use crate::{db::models::User, error::Result, schema::users};
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl};
use diesel::{OptionalExtension, QueryDsl};

use serde::Deserialize;

/// This struct will be used to deserialize the request body for the login route.
#[derive(Deserialize, Debug)]
pub struct LoginUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

pub fn authenticate_user(user: LoginUser, conn: &mut PgConnection) -> Result<User> {
    let table_user = if let Some(username) = user.username {
        // find the user in the database
        users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
            .optional()?
    } else if let Some(email) = user.email {
        // find the user in the database
        users::table
            .filter(users::email.eq(email))
            .first::<User>(conn)
            .optional()?
    } else {
        return Err(InvalidCredentials);
    };

    // check if the user exists
    let table_user = match table_user {
        Some(user) => user,
        None => return Err(UserNotFound),
    };

    // validate password with bcrypt
    if bcrypt::verify(user.password, &table_user.password_hash)? {
        return Ok(table_user);
    } else {
        return Err(InvalidCredentials);
    }
}
