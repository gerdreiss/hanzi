// @generated automatically by Diesel CLI.

diesel::table! {
    phrases (id) {
        id -> Integer,
        original -> Text,
        pinyin -> Text,
        translation -> Text,
    }
}

diesel::table! {
    settings (id) {
        id -> Integer,
        name -> Text,
        value -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    phrases,
    settings,
);
