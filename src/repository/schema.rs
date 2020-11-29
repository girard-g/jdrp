table! {
    caracter (id) {
        id -> Text,
        player_id -> Text,
        stats -> Text,
        gold -> Integer,
    }
}

table! {
    inventory (id) {
        id -> Text,
        caracter_id -> Text,
    }
}

table! {
    inventory_items_generated (inventory_id, items_generated_id) {
        inventory_id -> Text,
        items_generated_id -> Text,
        posx -> Integer,
        posy -> Integer,
    }
}

table! {
    items_generated (id) {
        id -> Text,
        item_type -> Text,
        equiped -> Integer,
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

allow_tables_to_appear_in_same_query!(
    caracter,
    inventory,
    inventory_items_generated,
    items_generated,
    players,
);
