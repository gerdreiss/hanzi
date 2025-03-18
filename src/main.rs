mod app;
mod llm;
mod model;
mod persistence;
mod schema;
mod screensize;
mod shortcuts;

use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::MigrationHarness;
use diesel_migrations::embed_migrations;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[tokio::main]
async fn main() -> eframe::Result {
    pretty_env_logger::init();

    let home_dir = std::env::var("HOME").expect("$HOME environment variable to exist");
    let hanzi_dir = format!("{}/.hanzi", home_dir);
    if !std::path::Path::new(&hanzi_dir).is_dir() {
        std::fs::create_dir_all(&hanzi_dir).expect("Successful folder creation");
    }

    let database_url = format!("{}/data.db", hanzi_dir);
    crate::persistence::create_connection(&database_url)
        .expect("Successful connection")
        .run_pending_migrations(MIGRATIONS)
        .expect("Successful migration");

    let screen_size = screensize::get_primary_screen_size()
        .map_err(|err| eframe::Error::AppCreation(Box::new(err)))?;

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
        Box::new(|cc| Ok(Box::new(app::HanziApp::new(cc, database_url)))),
    )
}
