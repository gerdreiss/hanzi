use diesel::associations::HasTable;
use diesel::prelude::*;

use crate::persistence::connection;
use crate::persistence::model;
use crate::persistence::read;

pub(crate) fn phrase(
    database_url: &str,
    text: String,
    language_name: String,
    language_code: String,
    text_translation: String,
    translation_romanization: Option<String>,
) -> Result<usize, super::PersistenceError> {
    use crate::schema::phrases::dsl::*;

    let mut conn = connection::create(database_url)?;

    let language_id = read::language_id(&mut conn, &language_code) //
        .or(language(&mut conn, &language_name, &language_code))?;

    let new_phrase = model::NewPhrase {
        lang_id: language_id,
        original: text,
        translation: text_translation,
        romanization: translation_romanization,
    };

    let result = diesel::insert_into(phrases::table())
        .values(&new_phrase)
        .execute(&mut conn)?;

    Ok(result)
}

pub(crate) fn language(
    conn: &mut SqliteConnection,
    language_name: &str,
    language_code: &str,
) -> Result<i32, super::PersistenceError> {
    use crate::schema::languages::dsl::*;

    let new_language = model::NewLanguage {
        name: language_name.to_string(),
        iso: language_code.to_string(),
    };

    let count = diesel::insert_into(languages::table())
        .values(new_language)
        .execute(conn)?;

    if count != 1 {
        Err(super::PersistenceError::Insert("languages".to_string()))
    } else {
        read::language_id(conn, language_code)
    }
}
