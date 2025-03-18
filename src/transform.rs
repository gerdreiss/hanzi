use crate::model;
use crate::persistence;

impl From<persistence::model::Language> for model::Language {
    fn from(value: persistence::model::Language) -> Self {
        Self {
            name: value.name,
            code: value.code,
        }
    }
}

impl From<(persistence::model::Language, persistence::model::Phrase)> for model::Phrase {
    fn from(value: (persistence::model::Language, persistence::model::Phrase)) -> Self {
        Self {
            text: value.1.text,
            language: model::Language::from(value.0),
            translation: value.1.translation,
            romanization: value.1.romanization.unwrap_or_default(),
        }
    }
}
