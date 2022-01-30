table! {
    credentials (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        credential_kind -> Integer,
        value -> Text,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        user_name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    credentials,
    users,
);
