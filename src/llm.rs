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
    #[error("LLM model not found")]
    ModelNotFound,
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
            LLMError::ModelNotFound => "LLM model not found".to_string(),
        }
    }
}

pub(crate) async fn query(query: Query) -> Result<model::Phrase, LLMError> {
    let ollama = Ollama::default();
    let model_name = get_model_name(&ollama).await?;
    let prompt = get_prompt(&query.text);

    log::debug!("Querying LLM model {} with prompt {}", model_name, prompt);

    let llm_response = query_llm(ollama, model_name, prompt).await?;

    log::debug!("LLM response: {}", llm_response);

    let json = extract_json_string(&llm_response)?;
    let response = serde_json::from_str::<model::Phrase>(json)?;
    Ok(response)
}

async fn get_model_name(ollama: &Ollama) -> Result<String, LLMError> {
    let models = ollama.list_local_models().await?;
    let model_name = if models.iter().any(|model| model.name == "llama3.2:latest") {
        Ok("llama3.2:latest".to_string())
    } else {
        log::warn!("It is recommended to install the 'llama3.2' model for best results");
        models
            .first()
            .map(|model| model.name.clone())
            .ok_or(LLMError::ModelNotFound)
    }?;
    log::info!("Found {} model", model_name);
    Ok(model_name)
}

fn get_prompt(request: &str) -> String {
    let prompt = format!(
        "Translate '{}' into English with pinyin. Format the result as JSON with the original text as element 'text', translation as element 'translation', and pinyin as element 'pinyin'",
        request
    );
    prompt
}

async fn query_llm(ollama: Ollama, model_name: String, prompt: String) -> Result<String, LLMError> {
    let llm_response = ollama
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
