// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        y1 -> Bytea,
        y2 -> Bytea,
    }
}