mod discord;
mod ollama;
mod utils;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut client = discord::new().await;
    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {why:?}");
    }
}
