// @generated automatically by Diesel CLI.

diesel::table! {
    member (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        salt -> Bpchar,
    }
}

diesel::table! {
    posting (id) {
        id -> Int4,
        posted_at -> Timestamp,
        check_number -> Nullable<Text>,
        summary -> Text,
        from_register_id -> Int4,
        to_register_id -> Int4,
        amount -> Int8,
    }
}

diesel::table! {
    register (id) {
        id -> Int4,
        title -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        parent_id -> Nullable<Int4>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    member,
    posting,
    register,
);
