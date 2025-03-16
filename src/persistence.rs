use diesel::associations::HasTable;
use diesel::prelude::*;
use thiserror::Error as ThisError;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::phrases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct Phrase {
    pub(crate) id: i32,
    pub(crate) lang_id: i32,
    pub(crate) original: String,
    pub(crate) translation: String,
    pub(crate) romanization: Option<String>,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::languages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Phrase))]
pub(crate) struct Language {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) iso: String,
}

#[derive(ThisError, Debug)]
pub(crate) enum PersistenceError {
    #[error("Connecting to database failed")]
    Connection(#[from] ConnectionError),
    #[error("Executing statement failed")]
    Execution(#[from] diesel::result::Error),
}

pub(crate) fn _create_connection(database_url: &str) -> Result<SqliteConnection, ConnectionError> {
    SqliteConnection::establish(database_url)
}

pub(crate) fn _create_phrase(
    conn: &mut SqliteConnection,
    phrase: Phrase,
) -> Result<usize, PersistenceError> {
    use crate::schema::phrases::dsl::*;

    let result = diesel::insert_into(phrases::table())
        .values(&phrase)
        .execute(conn)?;

    Ok(result)
}
