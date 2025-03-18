use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub(crate) struct Language {
    pub(crate) name: String,
    pub(crate) code: String,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub(crate) struct Phrase {
    pub(crate) text: String,
    pub(crate) language: Language,
    pub(crate) translation: String,
    pub(crate) romanization: String,
}
