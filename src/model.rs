use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub(crate) struct Phrase {
    pub(crate) text: String,
    pub(crate) translation: String,
    pub(crate) pinyin: String,
}
