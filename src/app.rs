use eframe::CreationContext;
use eframe::epaint::text::FontInsert;
use eframe::epaint::text::InsertFontFamily;
use egui::Widget;
use egui::os::OperatingSystem;
use egui_modal_spinner::ModalSpinner;
use egui_notify::Toasts;
use std::time::Duration;

use crate::llm;
use crate::shortcuts;

#[derive(Default)]
pub(crate) struct HanziApp {
    input: String,
    pinyin: String,
    translation: String,
    toasts: Toasts,
    spinner: ModalSpinner,
    is_macos: bool,
    llm_query: Option<poll_promise::Promise<Result<llm::Response, llm::LLMError>>>,
}

impl HanziApp {
    pub(crate) fn new(cc: &CreationContext<'_>) -> Self {
        cc.egui_ctx.add_font(FontInsert::new(
            "Han_Sans_CN_Light",
            egui::FontData::from_static(include_bytes!("../assets/Source Han Sans CN Light.otf")),
            vec![
                InsertFontFamily {
                    family: egui::FontFamily::Proportional,
                    priority: egui::epaint::text::FontPriority::Highest,
                },
                InsertFontFamily {
                    family: egui::FontFamily::Monospace,
                    priority: egui::epaint::text::FontPriority::Lowest,
                },
            ],
        ));
        Self {
            input: "学习汉语很有趣!".to_owned(),
            pinyin: "Xuéxí hànyǔ hěn yǒuqù!".to_owned(),
            translation: "Learning Chinese is fun!".to_owned(),
            toasts: Toasts::default().with_anchor(egui_notify::Anchor::BottomRight),
            spinner: ModalSpinner::new()
                .spinner_size(60.)
                .spinner_color(egui::Color32::YELLOW),
            llm_query: None,
            is_macos: cc.egui_ctx.os() == OperatingSystem::Mac,
        }
    }
}

impl eframe::App for HanziApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // CREATE UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.with_layout(
                    egui::Layout::left_to_right(egui::Align::TOP).with_main_justify(true),
                    |ui| {
                        egui::TextEdit::singleline(&mut self.input)
                            .id(egui::Id::new("hanzi_editor"))
                            .horizontal_align(egui::Align::Center)
                            .text_color(egui::Color32::YELLOW)
                            .margin(egui::Margin::same(16))
                            .font(egui::FontId::new(64., egui::FontFamily::Proportional))
                            .ui(ui)
                    },
                );
                egui::Frame::new().inner_margin(14.).show(ui, |ui| {
                    ui.columns_const(|[col_1, col_2]| {
                        col_1.vertical(|ui| ui.label(egui::RichText::new(&self.pinyin).size(28.)));
                        col_2.vertical(|ui| ui.label(egui::RichText::new(&self.translation).size(28.)));
                    })
                });
            });
        });

        // HANDLE EVENTS
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::save(self.is_macos))) {
            self.toasts
                .info("This is where the phrase with pinyin and translation will be saved")
                .duration(Some(Duration::from_secs(5)))
                .show_progress_bar(true);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::find(self.is_macos))) {
            self.toasts
                .info("This is where the search for phrases will open")
                .duration(Some(Duration::from_secs(5)))
                .show_progress_bar(true);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::settings(self.is_macos))) {
            self.toasts
                .info("This is where the settings will open")
                .duration(Some(Duration::from_secs(5)))
                .show_progress_bar(true);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::about(self.is_macos))) {
            self.toasts
                .info("This is where the about dialog will open")
                .duration(Some(Duration::from_secs(5)))
                .show_progress_bar(true);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::F1)) {
            self.toasts
                .info("This is where the help will be displayed")
                .duration(Some(Duration::from_secs(5)))
                .show_progress_bar(true);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) && self.llm_query.is_none() {
            let request = llm::Request {
                text: self.input.clone(),
            };
            self.llm_query = Some(poll_promise::Promise::spawn_async(llm::query(request)));
            self.spinner.open();
        }
        // HANDLE LLM QUERIES
        if let Some(query) = self.llm_query.take() {
            match query.try_take() {
                Ok(result) => match result {
                    Ok(response) => {
                        self.input = response.original.clone();
                        self.pinyin = response.pinyin.clone();
                        self.translation = response.translation.clone();
                        self.llm_query = None;
                        self.spinner.close();
                    }
                    Err(err) => {
                        self.llm_query = None;
                        self.spinner.close();
                        self.toasts
                            .error(format!("Async call to LLM failed: {}", err.cause()))
                            .duration(Some(Duration::from_secs(5)))
                            .show_progress_bar(true);
                        self.spinner.close();
                    }
                },
                Err(promise) => self.llm_query = Some(promise),
            }
        }

        self.spinner.update_with_content(ctx, |ui| {
            ui.label("Querying LLM...");
        });

        self.toasts.show(ctx);
    }
}
