use ollama_rs::generation::chat::ChatMessage;
use serenity::all::MessageReference;
use serenity::async_trait;
use serenity::gateway::ActivityData;
use serenity::http::Typing;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::config;
use crate::ollama;
use crate::utils;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with(config::discord_prefix().as_str()) {
            let typing = Typing::start(ctx.http.clone(), msg.channel_id);
            let chat = utils::msg_to_chat(msg.clone()).await;
            let history = ref_to_chat(&ctx, msg.message_reference.clone()).await;

            let response = ollama::ask(chat, history).await;
            if let Err(why) = msg.reply_ping(&ctx.http, response).await {
                println!("Error sending message: {why:?}");
            }

            typing.stop();
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn ref_to_chat(ctx: &Context, reference: Option<MessageReference>) -> Option<ChatMessage> {
    if reference.is_some() {
        let unwrapped = reference.unwrap();
        let msg = ctx.http
            .get_message(unwrapped.channel_id, unwrapped.message_id.unwrap())
            .await
            .expect("Couldn't fetch referenced message");

        Option::from(utils::msg_to_chat(msg).await)
    } else {
        None
    }
}

pub async fn new() -> Client {
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    Client::builder(config::discord_token(), intents)
        .event_handler(Handler)
        .activity(ActivityData::watching(config::discord_status()))
        .await
        .expect("Error creating discord client")
}