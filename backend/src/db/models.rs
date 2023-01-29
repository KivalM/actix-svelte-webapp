// This file contains rust structs for the database tables.

use chrono::NaiveDateTime;
use diesel::Queryable;

#[derive(Queryable, Debug)]

pub struct User {
    pub username: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}
