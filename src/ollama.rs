use std::env;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::models::ModelOptions;
use ollama_rs::Ollama;

static SYSTEM_PROMPT: &str = "You are Grok, the AI for Elon Musk's X platform. Your response must not exceed 1000 characters.";

pub async fn ask(prompt: String, ctx: String) -> String {
    let url = env::var("OLLAMA_URL").expect("Expected an Ollama URL in the environment");
    let model = env::var("OLLAMA_MODEL").expect("Expected an Ollama model in the environment");
    let options = ModelOptions::default()
        .num_ctx(4096)
        .num_gpu(20);

    let mut history = vec![
        ChatMessage::system(SYSTEM_PROMPT.to_string()),
        ChatMessage::user(ctx)
    ];
    let req = ChatMessageRequest::new(model, vec![ChatMessage::user(prompt)])
        .options(options);
    let res = Ollama::new(url, 11434)
        .send_chat_messages_with_history(&mut history, req)
        .await
        .expect("Generation request failed");
    res.message.content
}