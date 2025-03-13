use ollama_rs::Ollama;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::chat::request::ChatMessageRequest;

use crate::model;

const PROMPT: &str = r#"
provide the meaning and the pronunciation for the following text in JSON format using || as separator.
the first element should be labeled 'original' and contain the text.
the second element should be labeled 'translation' and contain the translation of the text.
the third element should be labeled 'pronunciation' and contain the pronunciation for the text
"#;

pub(crate) async fn _query(
    ollama: Ollama,
    request: model::Request,
) -> Result<String, ollama_rs::error::OllamaError> {
    let prompt = format!("{}: {}", PROMPT, request.text);

    ollama
        .send_chat_messages(ChatMessageRequest::new(
            request.model,
            vec![ChatMessage::user(prompt)],
        ))
        .await
        .map(|res| res.message.content)
}
