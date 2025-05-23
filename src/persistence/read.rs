use diesel::associations::HasTable;
use diesel::prelude::*;

use crate::persistence::database_connection;
use crate::persistence::model;

pub(crate) fn phrases(database_url: &str, term: &str) -> Result<Vec<model::Phrase>, super::PersistenceError> {
    use crate::persistence::schema::phrases::dsl::*;

    let mut conn = database_connection::create(database_url)?;

    let result = phrases::table()
        .filter(original.like(format!("%{}%", term)))
        .load(&mut conn)
        .inspect_err(|error| log::error!("Failed to load phrases by term {}: {:?}", term, error))?;

    Ok(result)
}

pub(crate) fn setting(database_url: &str, setting_name: &str) -> Result<model::Setting, super::PersistenceError> {
    use crate::persistence::schema::settings::dsl::*;

    let mut conn = database_connection::create(database_url)?;

    let result = settings::table()
        .filter(name.eq(setting_name))
        .first(&mut conn)
        .inspect_err(|error| log::error!("Failed to load setting by name {}: {:?}", setting_name, error))?;

    Ok(result)
}
