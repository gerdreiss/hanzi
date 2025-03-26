use diesel::associations::HasTable;
use diesel::prelude::*;

use crate::persistence::connection;
use crate::persistence::model;

pub(crate) fn phrase(
    database_url: &str,
    phrase_text: String,
    phrase_translation: String,
    phrase_pinyin: String,
) -> Result<usize, super::PersistenceError> {
    use crate::persistence::schema::phrases::dsl::*;

    let mut conn = connection::create(database_url)?;

    let new_phrase = model::NewPhrase {
        text: phrase_text,
        translation: phrase_translation,
        pinyin: phrase_pinyin,
    };

    let result = diesel::insert_into(phrases::table())
        .values(&new_phrase)
        .on_conflict(text)
        .do_update()
        .set(&new_phrase)
        .execute(&mut conn)?;

    Ok(result)
}
