// @generated automatically by Diesel CLI.

diesel::table! {
    languages (id) {
        id -> Integer,
        name -> Text,
        code -> Text,
    }
}

diesel::table! {
    phrases (id) {
        id -> Integer,
        language_id -> Integer,
        text -> Text,
        translation -> Text,
        romanization -> Nullable<Text>,
    }
}

diesel::joinable!(phrases -> languages (language_id));

diesel::allow_tables_to_appear_in_same_query!(languages, phrases,);
