use nonempty_collections::NEVec;
use ollama_rs::Ollama;
use ollama_rs::error::OllamaError;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::chat::request::ChatMessageRequest;

use crate::model;

pub(crate) struct Query {
    pub(crate) text: String,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum LLMError {
    #[error("LLM query failed")]
    LLMQuery(#[from] OllamaError),
    #[error("LLM response processing failed")]
    LLMResponse(#[from] serde_json::Error),
    #[error("Invalid JSON cound not be extracted: {0}")]
    InvalidJson(String),
    #[error("Local LLM models not found")]
    LocalModelNotFound,
    #[error("Environment variable not set")]
    EnvVar(#[from] std::env::VarError),
}

impl LLMError {
    pub(crate) fn cause(&self) -> String {
        match self {
            LLMError::LLMQuery(ollama_error) => match ollama_error {
                OllamaError::ToolCallError(error) => error.to_string(),
                OllamaError::JsonError(error) => error.to_string(),
                OllamaError::ReqwestError(error) => error.to_string(),
                OllamaError::InternalError(error) => error.message.clone(),
                OllamaError::Other(error) => error.clone(),
            },
            LLMError::LLMResponse(error) => error.to_string(),
            LLMError::InvalidJson(error) => error.to_string(),
            LLMError::LocalModelNotFound => "Local LLM model not found".to_string(),
            LLMError::EnvVar(error) => error.to_string(),
        }
    }
}

pub(crate) async fn list_local_model_names() -> Result<NEVec<String>, LLMError> {
    let ollama = Ollama::default();
    let models = ollama.list_local_models().await?;
    let model_names = models.iter().map(|model| model.name.clone()).collect();
    NEVec::try_from_vec(model_names).ok_or(LLMError::LocalModelNotFound)
}

pub(crate) async fn query(llm_model: String, query: Query) -> Result<model::Phrase, LLMError> {
    let prompt = get_prompt(&query.text);

    log::debug!("Querying LLM model {} with prompt {}", llm_model, prompt);

    let llm_response = query_llm(llm_model, prompt).await?;

    log::debug!("LLM response: {}", llm_response);

    let json = extract_json_string(&llm_response)?;
    let response = serde_json::from_str::<model::Phrase>(json)?;
    Ok(response)
}

fn get_prompt(request: &str) -> String {
    let prompt = r#"
Translate the following Chinese phrase into English and return the result as JSON containing the original text as 'original', it's Pinyin as 'pinyin', and the translation as 'translation'.

Chinese phrase: "#;

    prompt.to_owned() + request
}

async fn query_llm(model_name: String, prompt: String) -> Result<String, LLMError> {
    let llm_response = Ollama::default()
        .send_chat_messages(ChatMessageRequest::new(model_name, vec![ChatMessage::user(prompt)]))
        .await
        .map(|res| res.message.content)?;
    Ok(llm_response)
}

fn extract_json_string(s: &str) -> Result<&str, LLMError> {
    let start = s.find('{');
    let end = s.rfind('}');

    start
        .zip(end)
        .filter(|(start, end)| start < end)
        .map(|(start, end)| &s[start..end + 1])
        .ok_or(LLMError::InvalidJson(s.to_string()))
}
