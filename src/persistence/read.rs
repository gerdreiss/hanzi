use diesel::associations::HasTable;
use diesel::prelude::*;

use crate::persistence::connection;
use crate::persistence::model;

pub(crate) fn phrases(
    database_url: &str,
    term: &str,
) -> Result<Vec<(model::Language, model::Phrase)>, super::PersistenceError> {
    use crate::schema::languages::dsl::*;
    use crate::schema::phrases::dsl::*;

    let mut conn = connection::create(database_url)?;

    let result = languages::table()
        .inner_join(phrases::table())
        .filter(text.like(format!("%{}%", term)))
        .select((model::Language::as_select(), model::Phrase::as_select()))
        .load::<(model::Language, model::Phrase)>(&mut conn)?;

    Ok(result)
}

pub(super) fn language_id(
    conn: &mut SqliteConnection,
    language_code: &str,
) -> Result<i32, super::PersistenceError> {
    use crate::schema::languages::dsl::*;

    let lang = languages
        .filter(code.eq(language_code))
        .select(model::Language::as_select())
        .get_result(conn)?;

    Ok(lang.id)
}
