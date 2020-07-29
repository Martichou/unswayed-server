table! {
    access_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        access_token -> Text,
        refresh_token -> Text,
        created_at -> Timestamp,
        expire_at -> Timestamp,
    }
}

table! {
    images (id) {
        id -> Int4,
        user_id -> Int4,
        realname -> Text,
        fakedname -> Varchar,
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

allow_tables_to_appear_in_same_query!(access_tokens, images, users,);
