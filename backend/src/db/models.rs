// This file contains rust structs for the database tables.

use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

/// This struct represents the users table in the database.
/// We use this struct to deserialize the data from the database.
/// It does not implement Serialize because we don't want to send the password hash to the client.
///
/// Instead we use the DisplayableUser struct to send the data to the client.
#[derive(Queryable, Debug, Deserialize, Serialize)]

pub struct User {
    pub username: String,
    pub name: String,
    pub email: String,
    // we don't want to send the password hash to the client
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}
