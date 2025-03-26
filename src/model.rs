use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub(crate) struct Phrase {
    pub(crate) original: String,
    pub(crate) pinyin: String,
    pub(crate) translation: String,
}
