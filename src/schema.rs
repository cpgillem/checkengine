// @generated automatically by Diesel CLI.

diesel::table! {
    member (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}

diesel::table! {
    register (id) {
        id -> Int4,
        title -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    member,
    register,
);
