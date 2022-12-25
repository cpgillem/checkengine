// @generated automatically by Diesel CLI.

diesel::table! {
    register (id) {
        id -> Int4,
        title -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}
