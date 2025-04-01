mod app;
mod llm;
mod model;
mod persistence;
mod screensize;
mod shortcuts;
mod transform;
mod ui;

use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::embed_migrations;

use crate::persistence::migration;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[tokio::main]
async fn main() -> eframe::Result {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let database_path = migration::run(MIGRATIONS);

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
        Box::new(|cc| Ok(Box::new(app::HanziApp::new(cc, database_path)))),
    )
}
