table! {
    login_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Varchar,
        expiry -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        phone -> Varchar,
    }
}

joinable!(login_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(
    login_tokens,
    users,
);
