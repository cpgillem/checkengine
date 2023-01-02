// @generated automatically by Diesel CLI.

diesel::table! {
    member (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        salt -> Varchar,
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
        created_at -> Timestamp,
        modified_at -> Timestamp,
        posting_group_id -> Int4,
    }
}

diesel::table! {
    posting_group (id) {
        id -> Int4,
        posted_at -> Timestamp,
        check_number -> Nullable<Text>,
        summary -> Text,
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
        parent_id -> Nullable<Int4>,
    }
}

diesel::joinable!(posting -> posting_group (posting_group_id));

diesel::allow_tables_to_appear_in_same_query!(
    member,
    posting,
    posting_group,
    register,
);
