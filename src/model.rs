use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Language {
    pub(crate) name: String,
    pub(crate) iso_code: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Phrase {
    pub(crate) original: String,
    pub(crate) language: Language,
    pub(crate) translation: String,
    pub(crate) romanization: String,
}
