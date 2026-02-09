mod discord;
mod ollama;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let llm = ollama::setup();
    let mut client = discord::setup().await;
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
