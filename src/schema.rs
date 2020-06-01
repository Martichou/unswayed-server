table! {
    access_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        access_token -> Text,
        created_at -> Timestamp,
    }
}

table! {
    refresh_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        refresh_token -> Text,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        passwd -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    access_tokens,
    refresh_tokens,
    users,
);
