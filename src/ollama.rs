use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::models::ModelOptions;
use ollama_rs::Ollama;

use crate::config;

static MAX_LENGTH: usize = 2000;

pub async fn ask(msg: ChatMessage, hist: Option<ChatMessage>) -> String {
    let options = ModelOptions::default()
        .num_ctx(4096)
        .num_gpu(20);

    let mut res = Ollama::new(config::ollama_url(), 11434)
        .send_chat_messages_with_history(
            &mut vec![
                ChatMessage::system(config::ollama_system_prompt()),
                hist.unwrap_or(ChatMessage::user("".to_string()))], // TODO: adding an empty message when there's no history is a bit hacky
            ChatMessageRequest::new(config::ollama_model(), vec![msg])
                .options(options))
        .await
        .expect("Generation request failed");

    // Truncate response to ensure it doesn't exceed Discord's message length limit
    res.message.content.truncate(MAX_LENGTH);
    res.message.content
}