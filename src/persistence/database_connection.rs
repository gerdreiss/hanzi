use diesel::prelude::*;

pub(crate) fn create(database_url: &str) -> Result<SqliteConnection, super::PersistenceError> {
    let connection = SqliteConnection::establish(database_url)?;
    Ok(connection)
}
