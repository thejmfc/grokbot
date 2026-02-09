use std::env;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

pub fn setup() -> Ollama {
    let url = env::var("OLLAMA_URL").expect("Expected an Ollama URL in the environment");

    Ollama::new(url, 11434)
}

pub async fn ask(ollama: Ollama, prompt: String) -> String {
    let model = env::var("OLLAMA_MODEL").expect("Expected an Ollama model in the environment");
    let res = ollama.generate(GenerationRequest::new(model, prompt)).await.expect("Generation request failed");

    res.response
}