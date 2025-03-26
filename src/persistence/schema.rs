// @generated automatically by Diesel CLI.

diesel::table! {
    phrases (id) {
        id -> Integer,
        original -> Text,
        pinyin -> Text,
        translation -> Text,
    }
}
