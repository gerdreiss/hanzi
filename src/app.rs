use eframe::CreationContext;
use eframe::epaint::text::FontInsert;
use eframe::epaint::text::InsertFontFamily;
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

pub(crate) struct HanziApp {
    pub(crate) database_url: String,
    pub(crate) toasts: Toasts,
    pub(crate) spinner: ModalSpinner,
    pub(crate) is_macos: bool,
    pub(crate) input: String,
    pub(crate) llm_query: Option<Promise<Result<model::Phrase, llm::LLMError>>>,
    pub(crate) llm_query_start: Option<Instant>,
    pub(crate) phrase: Option<model::Phrase>,
    pub(crate) phrases: Vec<model::Phrase>,
    pub(crate) open_settings: bool,
    pub(crate) open_about: bool,
    pub(crate) open_help: bool,
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
            open_settings: false,
            open_about: false,
            open_help: false,
        }
    }
}

impl HanziApp {
    pub(crate) fn save_phrase(&mut self) {
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

    pub(crate) fn edit_translation_result(&mut self) {
        self.toasts
            .info("This is where the results can be edited")
            .duration(Some(Duration::from_secs(5)))
            .show_progress_bar(true);
    }

    pub(crate) fn learn(&mut self) {
        self.toasts
            .info("This is where the learning mask will open")
            .duration(Some(Duration::from_secs(5)))
            .show_progress_bar(true);
    }

    pub(crate) fn read_phrases(&mut self) {
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

    pub(crate) fn query_llm(&mut self) {
        self.phrase = None;
        self.llm_query = Some(Promise::spawn_async(llm::query(llm::Query {
            text: self.input.to_owned(),
        })));
        self.llm_query_start = Some(Instant::now());
        self.spinner.open();
    }

    pub(crate) fn handle_escape(&mut self) {
        if let Some(q) = self.llm_query.take() {
            q.abort();
            self.llm_query = None;
            self.llm_query_start = None;
            self.spinner.close();
        }
        if self.open_about {
            self.open_about = false;
        }
        if self.open_help {
            self.open_help = false;
        }
        if self.open_settings {
            self.open_settings = false;
        }
    }
}
