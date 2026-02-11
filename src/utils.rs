use base64::Engine;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::images::Image;
use reqwest::get;
use serenity::all::Message;

use crate::config;

pub async fn download_image(url: &str) -> Result<Image, reqwest::Error> {
    let response = get(url).await?;
    let bytes = response.bytes().await?;
    let base64_image = base64::engine::general_purpose::STANDARD.encode(&bytes);
    Ok(Image::from_base64(&base64_image))
}

pub async fn msg_to_chat(msg: Message) -> ChatMessage {
    // Strip away the command prefix if present, otherwise use the entire message
    let chat = ChatMessage::user(strip_prefix(msg.content));

    // Add the first attachment as an image, if present
    if msg.attachments.is_empty() {
        chat
    } else {
        let img = download_image(&msg.attachments[0].url).await.expect("Failed to download image");
        chat.add_image(img)
    }
}

pub fn strip_prefix(s: String) -> String {
    s.strip_prefix(config::discord_prefix().as_str())
        .unwrap_or(s.as_str())
        .to_string()
}