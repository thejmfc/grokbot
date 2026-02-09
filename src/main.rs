mod discord;
mod ollama;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut client = discord::setup().await;
    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {why:?}");
    }
}
