mod app;
mod hanzi_logging;
mod llm;
mod model;
mod persistence;
mod screensize;
mod settings;
mod shortcuts;
mod transform;
mod ui;

use diesel_migrations::embed_migrations;
use diesel_migrations::EmbeddedMigrations;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[tokio::main]
async fn main() -> eframe::Result {
    dotenv::dotenv().ok();
    hanzi_logging::init();

    let database_path = persistence::database_migration::run(MIGRATIONS)
        .expect("Database migration needs to run before start of the application");

    let local_llm_models = llm::list_local_model_names()
        .await
        .expect("At least one LLM model should be installed locally with Ollama.");

    let default_llm_model = local_llm_models
        .iter()
        .find(|model| model.starts_with("mistral"))
        .unwrap_or(local_llm_models.first());

    let selected_llm_model =
        settings::load_setting(&database_path, model::SettingName::LlmModel).unwrap_or(default_llm_model.to_owned());

    let screen_size = screensize::get_primary_screen_size();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([1200., 800.])
            .with_inner_size([screen_size.x as f32 * 0.6, screen_size.y as f32 * 0.7]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "学习汉字",
        options,
        Box::new(|cc| {
            Ok(Box::new(app::HanziApp::new(
                cc,
                local_llm_models.into(),
                selected_llm_model,
                database_path,
            )))
        }),
    )
}
