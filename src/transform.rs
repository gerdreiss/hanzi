use crate::model;
use crate::persistence;

impl From<persistence::model::Phrase> for model::Phrase {
    fn from(value: persistence::model::Phrase) -> Self {
        Self {
            original: value.original,
            pinyin: value.pinyin,
            translation: value.translation,
        }
    }
}
