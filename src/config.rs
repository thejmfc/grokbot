use std::env;

fn get_env_var(key: &str) -> String {
    env::var(key).expect(&format!("{} environment variable not set", key))
}

pub fn ollama_url() -> String {
    get_env_var("OLLAMA_URL")
}

pub fn ollama_model() -> String {
    get_env_var("OLLAMA_MODEL")
}

pub fn ollama_system_prompt() -> String {
    get_env_var("OLLAMA_SYSTEM_PROMPT")
}

pub fn discord_token() -> String {
    get_env_var("DISCORD_TOKEN")
}

pub fn discord_prefix() -> String {
    get_env_var("DISCORD_PREFIX")
}

pub fn discord_status() -> String {
    get_env_var("DISCORD_STATUS")
}