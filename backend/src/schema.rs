// @generated automatically by Diesel CLI.

diesel::table! {
    users (email) {
        email -> Varchar,
        username -> Varchar,
        name -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
    }
}
