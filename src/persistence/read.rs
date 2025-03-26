use diesel::associations::HasTable;
use diesel::prelude::*;

use crate::persistence::connection;
use crate::persistence::model;

pub(crate) fn phrases(database_url: &str, term: &str) -> Result<Vec<model::Phrase>, super::PersistenceError> {
    use crate::persistence::schema::phrases::dsl::*;

    let mut conn = connection::create(database_url)?;

    let result = phrases::table()
        .filter(original.like(format!("%{}%", term)))
        .load(&mut conn)?;

    Ok(result)
}
