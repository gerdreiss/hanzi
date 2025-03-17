use eframe::CreationContext;
use eframe::epaint::text::FontInsert;
use eframe::epaint::text::InsertFontFamily;
use egui::Widget;
use egui::os::OperatingSystem;
use egui_modal_spinner::ModalSpinner;
use egui_notify::Anchor;
use egui_notify::Toasts;
use poll_promise::Promise;
use std::time::Duration;

use crate::llm;
use crate::model;
use crate::persistence;
use crate::shortcuts;

pub(crate) struct HanziApp {
    database_url: String,
    toasts: Toasts,
    spinner: ModalSpinner,
    is_macos: bool,
    input: String,
    llm_query: Option<Promise<Result<model::Phrase, llm::LLMError>>>,
    phrase: Option<model::Phrase>,
}

impl HanziApp {
    pub(crate) fn new(cc: &CreationContext<'_>, database_url: String) -> Self {
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
            database_url,
            toasts: Toasts::default().with_anchor(Anchor::BottomRight),
            spinner: ModalSpinner::new()
                .spinner_size(60.)
                .spinner_color(egui::Color32::YELLOW),
            is_macos: cc.egui_ctx.os() == OperatingSystem::Mac,
            input: "学习汉语很有趣!".to_owned(),
            llm_query: None,
            phrase: None,
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
                egui::Frame::new().inner_margin(18.).show(ui, |ui| {
                    ui.columns_const(|[col_1, col_2]| {
                        col_2.vertical(|ui| {
                            ui.label(
                                egui::RichText::new(
                                    &self
                                        .phrase
                                        .as_ref()
                                        .map(|p| p.translation.clone())
                                        .unwrap_or_default(),
                                )
                                .size(28.),
                            )
                        });
                        col_1.vertical(|ui| {
                            ui.label(
                                egui::RichText::new(
                                    &self
                                        .phrase
                                        .as_ref()
                                        .map(|p| p.romanization.clone())
                                        .unwrap_or_default(),
                                )
                                .size(28.),
                            )
                        });
                    })
                });
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    egui::Hyperlink::from_label_and_url("egui", "https://github.com/emilk/egui").ui(ui);
                    egui::Label::new("Powered by").ui(ui);
                    ui.separator();
                });
            });
        });

        // HANDLE EVENTS
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::save(self.is_macos))) {
            if let Some(phrase) = self.phrase.as_mut() {
                match persistence::create_phrase(
                    &self.database_url,
                    phrase.original.clone(),
                    phrase.language.name.clone(),
                    phrase.language.iso_code.clone(),
                    phrase.translation.clone(),
                    Some(phrase.romanization.clone()),
                ) {
                    Ok(_) => self
                        .toasts
                        .info("Phrase saved successfully")
                        .duration(Some(Duration::from_secs(5)))
                        .show_progress_bar(true),
                    Err(err) => {
                        log::error!("{}", err);
                        self.toasts
                            .error("Phrase could not be saved")
                            .duration(Some(Duration::from_secs(5)))
                            .show_progress_bar(true)
                    }
                };
            } else {
                self.toasts
                    .error("Nothing to save")
                    .duration(Some(Duration::from_secs(5)))
                    .show_progress_bar(true);
            }
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
            self.phrase = None;
            self.llm_query = Some(Promise::spawn_async(llm::query(llm::Query {
                text: self.input.to_owned(),
            })));
            self.spinner.open();
        }

        // HANDLE LLM QUERIES
        if let Some(query) = self.llm_query.take() {
            match query.try_take() {
                Ok(Ok(response)) => {
                    self.llm_query = None;
                    self.spinner.close();
                    self.input = response.original.clone();
                    self.phrase = Some(response);
                }
                Ok(Err(err)) => {
                    log::error!(
                        "Error occurred when querying LLM: {} caused by {}",
                        err,
                        err.cause()
                    );
                    self.llm_query = None;
                    self.spinner.close();
                    self.toasts
                        .error(format!("Querying LLM failed: {}", err.cause()))
                        .duration(Some(Duration::from_secs(5)))
                        .show_progress_bar(true);
                }
                Err(promise) => self.llm_query = Some(promise),
            }
        }

        self.spinner.update_with_content(ctx, |ui| {
            ui.label("Querying LLM...");
        });

        self.toasts.show(ctx);
    }
}
