use diesel::associations::HasTable;
use diesel::prelude::*;
use thiserror::Error as ThisError;

use crate::schema::languages;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::phrases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct Phrase {
    pub(crate) id: i32,
    pub(crate) lang_id: i32,
    pub(crate) original: String,
    pub(crate) translation: String,
    pub(crate) romanization: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::phrases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct NewPhrase {
    lang_id: i32,
    original: String,
    translation: String,
    romanization: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::languages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Phrase))]
struct Language {
    id: i32,
    name: String,
    iso: String,
}

#[derive(Insertable)]
#[diesel(table_name = languages)]
struct NewLanguage {
    name: String,
    iso: String,
}

#[derive(ThisError, Debug)]
pub(crate) enum PersistenceError {
    #[error("Connecting to database failed")]
    Connection(#[from] ConnectionError),
    #[error("Executing statement failed")]
    Execution(#[from] diesel::result::Error),
    #[error("Insert into {0} failed")]
    Insert(String),
    #[error("Not found")]
    NotFound,
}

pub(crate) fn _create_connection(database_url: &str) -> Result<SqliteConnection, ConnectionError> {
    SqliteConnection::establish(database_url)
}

pub(crate) fn _find_phrases(
    conn: &mut SqliteConnection,
    text: String,
) -> Result<Vec<Phrase>, PersistenceError> {
    use crate::schema::phrases::dsl::*;

    let result = phrases
        .filter(
            original
                .like(format!("%{}%", text))
                .or(translation.like(format!("%{}%", text)))
                .or(romanization.like(format!("%{}%", text))),
        )
        .limit(1)
        .select(Phrase::as_select())
        .load(conn)?;

    Ok(result)
}

pub(crate) fn _create_phrase(
    conn: &mut SqliteConnection,
    text: String,
    language_name: String,
    language_code: String,
    text_translation: String,
    translation_romanization: Option<String>,
) -> Result<usize, PersistenceError> {
    use crate::schema::phrases::dsl::*;

    let language_id = _get_language_id(conn, &language_code) //
        .or(_create_language(conn, &language_name, &language_code))?;

    let new_phrase = NewPhrase {
        lang_id: language_id,
        original: text,
        translation: text_translation,
        romanization: translation_romanization,
    };

    let result = diesel::insert_into(phrases::table())
        .values(&new_phrase)
        .execute(conn)?;

    Ok(result)
}

fn _get_language_id(conn: &mut SqliteConnection, language_code: &str) -> Result<i32, PersistenceError> {
    use crate::schema::languages::dsl::*;

    let result = languages
        .filter(iso.eq(language_code))
        .limit(1)
        .select(Language::as_select())
        .load(conn)?;

    result.first().map(|r| r.id).ok_or(PersistenceError::NotFound)
}

fn _create_language(
    conn: &mut SqliteConnection,
    language_name: &str,
    language_code: &str,
) -> Result<i32, PersistenceError> {
    use crate::schema::languages::dsl::*;

    let new_language = NewLanguage {
        name: language_name.to_string(),
        iso: language_code.to_string(),
    };

    let count = diesel::insert_into(languages::table())
        .values(new_language)
        .execute(conn)?;

    if count != 1 {
        Err(PersistenceError::Insert("languages".to_string()))
    } else {
        _get_language_id(conn, language_code)
    }
}
