use diesel::associations::HasTable;
use diesel::prelude::*;

use crate::persistence::connection;
use crate::persistence::model;

pub(crate) fn phrases(database_url: &str, term: &str) -> Result<Vec<model::Phrase>, super::PersistenceError> {
    use crate::persistence::schema::phrases::dsl::*;

    let mut conn = connection::create(database_url)?;

    let result = phrases::table()
        .filter(original.like(format!("%{}%", term)))
        .load(&mut conn)
        .inspect_err(|error| log::error!("Failed to load phrases by term {}: {:?}", term, error))?;

    Ok(result)
}

pub(crate) fn setting(database_url: &str, setting_name: &str) -> Result<Vec<model::Setting>, super::PersistenceError> {
    use crate::persistence::schema::settings::dsl::*;

    let mut conn = connection::create(database_url)?;

    let result = settings::table()
        .filter(name.eq(setting_name))
        .load(&mut conn)
        .inspect_err(|error| log::error!("Failed to load setting by name {}: {:?}", setting_name, error))?;

    Ok(result)
}
