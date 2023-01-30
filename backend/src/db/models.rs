// This file contains rust structs for the database tables.

use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

/// This struct represents the users table in the database.
/// We use this struct to deserialize the data from the database.
/// It does not implement Serialize because we don't want to send the password hash to the client.
///
/// Instead we use the DisplayableUser struct to send the data to the client.
#[derive(Queryable, Debug)]

pub struct User {
    pub username: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

/// This is the struct that we use to send the user data to the client.
///
/// It implements Serialize so that we can send it to the client.
/// It implements From<User> so that we can convert a User struct to a DisplayableUser struct.
#[derive(Serialize, Deserialize)]
pub struct DisplayableUser {
    pub username: String,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

impl From<User> for DisplayableUser {
    fn from(user: User) -> Self {
        DisplayableUser {
            username: user.username,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
        }
    }
}
