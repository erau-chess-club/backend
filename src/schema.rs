table! {
    games (id) {
        id -> Integer,
        white_id -> Integer,
        black_id -> Integer,
        white_points -> Float,
        black_points -> Float,
        pgn -> Nullable<Text>,
        scorecard_image -> Nullable<Binary>,
        game_end -> Timestamp,
        game_entered -> Timestamp,
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
