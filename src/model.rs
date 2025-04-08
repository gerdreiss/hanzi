use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error as ThisError;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub(crate) struct Phrase {
    pub(crate) original: String,
    pub(crate) pinyin: String,
    pub(crate) translation: String,
}

#[derive(ThisError, Debug)]
pub(crate) enum SettingError {
    #[error("Unknown setting: {0}")]
    Unknown(String),
    #[error("Failed to load setting: {0}")]
    Load(String),
    #[error("What??? {0}")]
    What(String),
}

pub(crate) enum SettingName {
    LLM_MODEL,
}

pub(crate) struct Setting {
    pub(crate) name: SettingName,
    pub(crate) value: String,
}

impl Display for SettingName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingName::LLM_MODEL => write!(f, "llm_model"),
        }
    }
}

impl FromStr for SettingName {
    type Err = SettingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "llm_model" => Ok(SettingName::LLM_MODEL),
            other => Err(SettingError::Unknown(other.to_string())),
        }
    }
}
