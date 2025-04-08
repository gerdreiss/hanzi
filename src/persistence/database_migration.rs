use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::MigrationHarness;

pub(crate) fn run(migrations: EmbeddedMigrations) -> Result<String, super::PersistenceError> {
    let home_dir = std::env::var("HOME").expect("$HOME environment variable to exist");
    let hanzi_dir = format!("{}/.hanzi", home_dir);

    if !std::path::Path::new(&hanzi_dir).exists() {
        std::fs::create_dir_all(&hanzi_dir).expect("Successful folder creation");
    }

    let database_path = format!("{}/data.db", hanzi_dir);

    let mut connection = super::database_connection::create(&database_path)?;

    connection
        .run_pending_migrations(migrations)
        .map_err(|err| super::PersistenceError::Migration(err.to_string()))?;

    Ok(database_path)
}
