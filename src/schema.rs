table! {
    poems (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        contents -> Text,
        is_public -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(poems);
