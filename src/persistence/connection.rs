use diesel::prelude::*;

pub(crate) fn create(database_url: &str) -> Result<diesel::SqliteConnection, super::PersistenceError> {
    let connection = diesel::SqliteConnection::establish(database_url)?;
    Ok(connection)
}
