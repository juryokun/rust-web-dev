// @generated automatically by Diesel CLI.

diesel::table! {
    users (username) {
        username -> Varchar,
        password -> Varchar,
    }
}
