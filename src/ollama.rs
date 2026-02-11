use std::env;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::models::ModelOptions;
use ollama_rs::Ollama;

static SYSTEM_PROMPT: &str = "You are Grok, the AI for Elon Musk's X platform. Your response must not exceed 1000 characters.";
static MAX_LENGTH: usize = 2000;

pub async fn ask(msg: ChatMessage, hist: Option<ChatMessage>) -> String {
    let url = env::var("OLLAMA_URL").expect("Expected an Ollama URL in the environment");
    let model = env::var("OLLAMA_MODEL").expect("Expected an Ollama model in the environment");
    let options = ModelOptions::default()
        .num_ctx(4096)
        .num_gpu(20);

    let mut res = Ollama::new(url, 11434)
        .send_chat_messages_with_history(
            &mut vec![
                ChatMessage::system(SYSTEM_PROMPT.to_string()),
                hist.unwrap_or(ChatMessage::user("".to_string()))],
            ChatMessageRequest::new(model, vec![msg])
                .options(options))
        .await
        .expect("Generation request failed");

    // Truncate response to ensure it doesn't exceed Discord's message length limit
    (&mut res.message.content).truncate(MAX_LENGTH);
    res.message.content
}