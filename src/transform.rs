use crate::model;
use crate::persistence;

impl From<persistence::model::Phrase> for model::Phrase {
    fn from(value: persistence::model::Phrase) -> Self {
        Self {
            text: value.text,
            translation: value.translation,
            pinyin: value.pinyin,
        }
    }
}
