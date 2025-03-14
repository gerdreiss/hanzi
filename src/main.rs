mod app;
mod llm;
mod shortcuts;

#[tokio::main]
async fn main() -> eframe::Result {
    pretty_env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "学习汉字",
        options,
        Box::new(|cc| Ok(Box::new(app::HanziApp::new(cc)))),
    )
}
