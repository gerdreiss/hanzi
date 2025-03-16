// @generated automatically by Diesel CLI.

diesel::table! {
    languages (id) {
        id -> Integer,
        name -> Text,
        iso -> Text,
    }
}

diesel::table! {
    phrases (id) {
        id -> Integer,
        original -> Text,
        lang_id -> Integer,
        translation -> Text,
        romanization -> Nullable<Text>,
    }
}

diesel::joinable!(phrases -> languages (lang_id));

diesel::allow_tables_to_appear_in_same_query!(
    languages,
    phrases,
);
