use diesel::associations::HasTable;
use diesel::prelude::*;

use crate::persistence::connection;
use crate::persistence::model;

pub(crate) fn phrase(
    database_url: &str,
    phrase_text: &str,
    phrase_pinyin: &str,
    phrase_translation: &str,
) -> Result<usize, super::PersistenceError> {
    use crate::persistence::schema::phrases::dsl::*;

    let mut conn = connection::create(database_url)?;

    let new_phrase = model::NewPhrase {
        original: phrase_text.to_owned(),
        pinyin: phrase_pinyin.to_owned(),
        translation: phrase_translation.to_owned(),
    };

    let result = diesel::insert_into(phrases::table())
        .values(&new_phrase)
        .on_conflict(original)
        .do_update()
        .set(&new_phrase)
        .execute(&mut conn)
        .inspect_err(|error| log::error!("Failed to upsert phrase {}: {:?}", phrase_text, error))?;

    Ok(result)
}
