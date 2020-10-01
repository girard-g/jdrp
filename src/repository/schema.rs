table! {
    caracter (id) {
        id -> Text,
        player_id -> Text,
        stats -> Text,
    }
}

table! {
    players (id) {
        id -> Text,
        pseudo -> Text,
        password -> Text,
        is_mj -> Integer,
    }
}

table! {
    posts (id) {
        id -> Nullable<Integer>,
        author -> Text,
        body -> Text,
        published_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    caracter,
    players,
    posts,
);
