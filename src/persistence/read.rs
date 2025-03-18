use diesel::prelude::*;

use crate::persistence::connection;
use crate::persistence::model;

pub(crate) fn phrases(
    database_url: &str,
    text: String,
) -> Result<Vec<model::Phrase>, super::PersistenceError> {
    use crate::schema::phrases::dsl::*;

    let mut conn = connection::create(database_url)?;

    let result = phrases
        .filter(
            original
                .like(format!("%{}%", text))
                .or(translation.like(format!("%{}%", text)))
                .or(romanization.like(format!("%{}%", text))),
        )
        .limit(1)
        .select(model::Phrase::as_select())
        .load(&mut conn)?;

    Ok(result)
}

pub(super) fn language_id(
    conn: &mut SqliteConnection,
    language_code: &str,
) -> Result<i32, super::PersistenceError> {
    use crate::schema::languages::dsl::*;

    let result = languages
        .filter(iso.eq(language_code))
        .limit(1)
        .select(model::Language::as_select())
        .load(conn)?;

    result
        .first()
        .map(|r| r.id)
        .ok_or(super::PersistenceError::NotFound)
}
