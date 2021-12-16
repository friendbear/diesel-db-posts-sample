table! {
    confirmations (id) {
        id -> Uuid,
        email -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        hash -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    confirmations,
    posts,
    users,
);
