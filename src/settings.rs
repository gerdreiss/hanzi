use crate::model;
use crate::persistence;

pub(crate) fn load_setting(
    database_url: &str,
    setting_name: model::SettingName,
) -> Result<String, model::SettingError> {
    persistence::read::setting(database_url, &setting_name.to_string())
        .map_err(|err| match err {
            persistence::PersistenceError::Migration(migration_error) => {
                log::error!("What???: {}", migration_error);
                model::SettingError::What(migration_error.to_string())
            }
            persistence::PersistenceError::Connection(connection_error) => {
                log::error!("No connection to database: {}", connection_error);
                model::SettingError::Load(format!("No connection to database: {}", connection_error))
            }
            persistence::PersistenceError::Execution(execution_error) => {
                log::error!("Failed to load: {}", execution_error);
                model::SettingError::Load(format!("Failed to load: {}", execution_error))
            }
        })
        .map(|setting| setting.value)
}
