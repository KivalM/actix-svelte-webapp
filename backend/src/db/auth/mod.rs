use super::models::User;
use crate::{error::Result, schema::users};
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::{PgConnection, QueryDsl};

pub mod login;
pub mod register;

/// This function will get a user by their email.
pub fn get_user_by_email(email: String, conn: &mut PgConnection) -> Result<User> {
    users::table
        .filter(users::email.eq(email))
        .first::<User>(conn)
        .map_err(|e| e.into())
}
