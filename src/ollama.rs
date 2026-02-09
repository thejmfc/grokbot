use std::env;

use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

pub async fn ask(prompt: String) -> String {
    let url = env::var("OLLAMA_URL").expect("Expected an Ollama URL in the environment");
    let model = env::var("OLLAMA_MODEL").expect("Expected an Ollama model in the environment");
    let res = Ollama::new(url, 11434)
        .generate(GenerationRequest::new(model, prompt))
        .await
        .expect("Generation request failed");

    res.response
}