use crate::error::CrateError::*;
use crate::{db::models::User, error::Result, schema::users};
use diesel::ExpressionMethods;
use diesel::{Insertable, PgConnection, QueryDsl, RunQueryDsl};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsertUser {
    pub username: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

pub fn register_user(user: RegisterUser, conn: &mut PgConnection) -> Result<User> {
    // check if the email is already taken and return an error if it is
    //
    // There is probably a more robust way to do with diesel insert_into and
    // catch the error, but I don't know how to do that yet.
    let email_taken = users::table
        .filter(users::email.eq(&user.email))
        .first::<User>(conn)
        .is_ok();

    if email_taken {
        return Err(EmailIsTaken);
    }

    // do the same for the username
    let username_taken = users::table
        .filter(users::username.eq(&user.username))
        .first::<User>(conn)
        .is_ok();

    if username_taken {
        return Err(UsernameIsTaken);
    }

    // hash the password
    let password_hash = bcrypt::hash(&user.password, 12)?;

    // create a new user using the insertable struct
    // we could use traits to make this more generic
    let user = InsertUser {
        username: user.username,
        name: user.name,
        email: user.email,
        password_hash,
    };

    let table_user = diesel::insert_into(users::table)
        .values(user)
        .get_results::<User>(conn)?
        // get the first element of the vector because we only inserted one user
        .remove(0);

    Ok(table_user)
}
