mod app;
mod llm;
mod model;
mod persistence;
mod schema;
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
    let database_url = format!("{}/data.db", hanzi_dir);
    if !std::fs::exists(&hanzi_dir).is_ok_and(|exists| exists) {
        std::fs::create_dir(&hanzi_dir).expect("Successful folder creation");
    }
    let mut connection =
        crate::persistence::create_connection(&database_url).expect("Successful connection");
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Successful migration");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "学习汉字",
        options,
        Box::new(|cc| Ok(Box::new(app::HanziApp::new(cc, database_url)))),
    )
}
