table! {
    games (id) {
        id -> Integer,
        white_id -> Integer,
        black_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        hash -> Text,
        erau_id -> Nullable<Integer>,
        signup_date -> Timestamp,
        is_officer -> Bool,
        chess_com_username -> Text,
        email -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    games,
    users,
);
