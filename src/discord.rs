use std::env;

use serenity::async_trait;
use serenity::gateway::ActivityData;
use serenity::http::Typing;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::ollama;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let bot_id = ctx.cache.current_user().id.to_string();
        let prefix = format!("<@{bot_id}>");

        if msg.content.starts_with(&prefix) {
            let typing = Typing::start(ctx.http.clone(), msg.channel_id);

            let prompt = msg.content.strip_prefix(&prefix).unwrap().to_string();
            let response = ollama::ask(prompt).await;

            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                println!("Error sending message: {why:?}");
            }

            typing.stop();
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn setup() -> Client {
    let token = env::var("DISCORD_TOKEN").expect("Expected a Discord token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    Client::builder(&token, intents)
        .event_handler(Handler)
        .activity(ActivityData::watching("Watching Israel bomb a school 🍿"))
        .await
        .expect("Err creating client")
}