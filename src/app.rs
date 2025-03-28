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
use std::time::Instant;

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
    llm_query_start: Option<Instant>,
    phrase: Option<model::Phrase>,
    phrases: Vec<model::Phrase>,
    open_about: bool,
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
            input: "学习汉语很有趣！".to_owned(),
            llm_query: None,
            llm_query_start: None,
            phrase: None,
            phrases: Vec::new(),
            open_about: false,
        }
    }

    fn input(&mut self, ui: &mut egui::Ui) {
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
    }

    fn translation(&mut self, ui: &mut egui::Ui) {
        ui.columns_const(|[col_1, col_2]| {
            col_2.vertical(|ui| {
                ui.label(
                    egui::RichText::new(self.phrase.as_ref().map(|p| p.translation.clone()).unwrap_or_default())
                        .size(28.),
                )
            });
            col_1.vertical(|ui| {
                ui.label(
                    egui::RichText::new(self.phrase.as_ref().map(|p| p.pinyin.clone()).unwrap_or_default()).size(28.),
                )
            });
        });
    }

    fn translations(&mut self, ui: &mut egui::Ui) {
        self.phrases.iter().for_each(|phrase| {
            ui.columns_const(|[col_1, col_2, col_3]| {
                col_3.vertical(|ui| ui.label(egui::RichText::new(phrase.translation.clone()).size(28.)));
                col_2.vertical(|ui| ui.label(egui::RichText::new(phrase.pinyin.clone()).size(28.)));
                col_1.vertical_centered_justified(|ui| {
                    ui.label(
                        egui::RichText::new(phrase.original.clone())
                            .color(egui::Color32::YELLOW)
                            .size(44.),
                    )
                });
            });
        });
    }
}

impl eframe::App for HanziApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // CREATE UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                self.input(ui);
                egui::Frame::NONE.inner_margin(18.).show(ui, |ui| {
                    if self.phrase.is_some() {
                        self.translation(ui);
                    } else if !self.phrases.is_empty() {
                        self.translations(ui);
                    }
                });
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    egui::Hyperlink::from_label_and_url("egui", "https://github.com/emilk/egui").ui(ui);
                    egui::Label::new("Powered by").ui(ui);
                    ui.separator();
                });
            });
        });

        // HANDLE EVENTS
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::edit(self.is_macos))) {
            self.toasts
                .info("This is where the results can be edited")
                .duration(Some(Duration::from_secs(5)))
                .show_progress_bar(true);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::learn(self.is_macos))) {
            self.toasts
                .info("This is where the learning mask will open")
                .duration(Some(Duration::from_secs(5)))
                .show_progress_bar(true);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::save(self.is_macos))) {
            if let Some(phrase) = self.phrase.as_mut() {
                match persistence::write::phrase(
                    &self.database_url,
                    &phrase.original,
                    &phrase.pinyin,
                    &phrase.translation,
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
            match persistence::read::phrases(&self.database_url, &self.input) {
                Ok(phrases) => {
                    if phrases.is_empty() {
                        self.toasts
                            .info("Nothing found")
                            .duration(Some(Duration::from_secs(5)))
                            .show_progress_bar(true);
                    } else if phrases.len() == 1 {
                        self.phrase = phrases
                            .into_iter()
                            .map(model::Phrase::from)
                            .collect::<Vec<_>>()
                            .first()
                            .cloned();
                        self.phrases = Vec::new();
                    } else {
                        self.phrase = None;
                        self.phrases = phrases.into_iter().map(model::Phrase::from).collect();
                    }
                }
                Err(err) => {
                    log::error!("Failed to load phrases: {}", err);
                    self.toasts
                        .info("Failed to load phrases")
                        .duration(Some(Duration::from_secs(5)))
                        .show_progress_bar(true);
                }
            }
        }
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::settings(self.is_macos))) {
            self.toasts
                .info("This is where the settings will open")
                .duration(Some(Duration::from_secs(5)))
                .show_progress_bar(true);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&shortcuts::about(self.is_macos))) {
            self.open_about = !self.open_about;
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
            self.llm_query_start = Some(Instant::now());
            self.spinner.open();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            if let Some(q) = self.llm_query.take() {
                q.abort();
                self.llm_query = None;
                self.llm_query_start = None;
                self.spinner.close();
            }
            if self.open_about {
                self.open_about = false;
            }
        }

        // HANDLE LLM QUERIES
        if let Some(query) = self.llm_query.take() {
            match query.try_take() {
                Ok(Ok(response)) => {
                    self.llm_query = None;
                    self.llm_query_start = None;
                    self.spinner.close();
                    self.input = response.original.clone();
                    self.phrase = Some(response);
                }
                Ok(Err(err)) => {
                    log::error!("Error occurred when querying LLM: {} caused by {}", err, err.cause());
                    self.llm_query = None;
                    self.llm_query_start = None;
                    self.spinner.close();
                    self.toasts
                        .error(format!("Querying LLM failed: {}", err.cause()))
                        .duration(Some(Duration::from_secs(5)))
                        .show_progress_bar(true);
                }
                Err(promise) => {
                    if let Some(start) = self.llm_query_start {
                        if start.elapsed().as_secs() > 60 {
                            promise.abort();
                            self.llm_query = None;
                            self.llm_query_start = None;
                            self.spinner.close();
                            self.toasts
                                .error("LLM query timed out")
                                .duration(Some(Duration::from_secs(5)))
                                .show_progress_bar(true);
                        } else {
                            self.llm_query = Some(promise)
                        }
                    } else {
                        self.llm_query = Some(promise)
                    }
                }
            }
        }

        if self.open_about {
            egui::Window::new("About").auto_sized().show(ctx, |ui| {
                egui::Frame::NONE.inner_margin(18.).show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("Hanzi").size(20.).color(egui::Color32::YELLOW));
                        ui.label(egui::RichText::new("A little helper for Chinese learners").size(20.));
                        ui.label(egui::RichText::new("Copyright (c) 2025, Gerd Reiss").size(20.));
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Written In").size(20.));
                            egui::Hyperlink::from_label_and_url(
                                egui::RichText::new("Rust").size(20.),
                                "https://www.rust-lang.org",
                            )
                            .ui(ui);
                        });
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Powered by").size(20.));
                            egui::Hyperlink::from_label_and_url(
                                egui::RichText::new("Egui").size(20.),
                                "https://github.com/emilk/egui",
                            )
                            .ui(ui);
                            ui.label(egui::RichText::new("and").size(20.));
                            egui::Hyperlink::from_label_and_url(
                                egui::RichText::new("Ollama").size(20.),
                                "https://ollama.com/",
                            )
                            .ui(ui);
                        });
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Hosted on").size(20.));
                            egui::Hyperlink::from_label_and_url(
                                egui::RichText::new("Github").size(20.),
                                "https://github.com/gerdreiss/hanzi",
                            )
                            .ui(ui);
                        });
                    });
                });
            });
        }

        self.spinner.update_with_content(ctx, |ui| {
            ui.label("Querying LLM...");
        });

        self.toasts.show(ctx);
    }
}
