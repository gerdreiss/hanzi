use ollama_rs::Ollama;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub(crate) struct Response {
    pub(crate) text: String,
    pub(crate) language: String,
    pub(crate) translation: String,
    pub(crate) pronunciation: String,
}

pub(crate) async fn query(ollama: &Ollama, request: Request) -> anyhow::Result<Response> {
    let prompt = format!("{}: {}", PROMPT, request.text);

    let llm_response = ollama
        .send_chat_messages(ChatMessageRequest::new(
            request.model,
            vec![ChatMessage::user(prompt)],
        ))
        .await
        .map(|res| res.message.content)?;

    let response: Response = serde_json::from_str(&llm_response)?;

    Ok(response)
}
