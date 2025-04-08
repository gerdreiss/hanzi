pub mod database_connection;
pub mod database_migration;
pub mod model;
pub mod read;
pub mod schema;
pub mod write;

use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub(crate) enum PersistenceError {
    #[error("Connecting to database failed")]
    Connection(#[from] diesel::ConnectionError),
    #[error("Database migration failed: {0}")]
    Migration(String),
    #[error("Executing statement failed")]
    Execution(#[from] diesel::result::Error),
}
