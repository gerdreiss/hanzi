// @generated automatically by Diesel CLI.

diesel::table! {
    phrases (id) {
        id -> Integer,
        text -> Text,
        translation -> Text,
        pinyin -> Text,
    }
}
