use ollama_rs::Ollama;
use ollama_rs::error::OllamaError;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Error as JsonError;
use thiserror::Error as ThisError;

const PROMPT: &str = r#"
provide the meaning and the pronunciation for the following text in JSON format.
the first element should be labeled 'original' and contain the text.
the second element should be labeled 'language' and contain the language of the text.
the third element should be labeled 'translation' and contain the translation of the text.
the fourth element should be labeled 'pronunciation' and contain the pronunciation for the text
"#;

pub(crate) struct Request {
    pub(crate) model: String,
    pub(crate) text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Response {
    pub(crate) text: String,
    pub(crate) language: String,
    pub(crate) translation: String,
    pub(crate) pronunciation: String,
}

#[derive(ThisError, Debug)]
pub(crate) enum LLMError {
    #[error("LLM query failed")]
    LLMQuery(#[from] OllamaError),
    #[error("LLM response processing failed")]
    LLMResponse(#[from] JsonError),
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
        }
    }
}

pub(crate) async fn query(request: Request) -> Result<Response, LLMError> {
    let prompt = format!("{}: {}", PROMPT, request.text);

    let llm_response = Ollama::default()
        .send_chat_messages(ChatMessageRequest::new(
            request.model,
            vec![ChatMessage::user(prompt)],
        ))
        .await
        .map(|res| res.message.content)?;

    let response = serde_json::from_str::<Response>(&llm_response)?;

    Ok(response)
}
