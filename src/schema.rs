table! {
    credentials (id) {
        id -> Integer,
        user_id -> Integer,
        credential_kind -> Integer,
        value -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        user_name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    credentials,
    users,
);
