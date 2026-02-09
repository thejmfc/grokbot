use std::env;
use std::string::ToString;
use serenity::async_trait;
use serenity::gateway::ActivityData;
use serenity::http::Typing;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::ollama;

struct Handler;

const PREFIX: &str = "<@1470419785576878164> ";

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with(&PREFIX) {
            let typing = Typing::start(ctx.http.clone(), msg.channel_id);
            let prompt = get_prompt(&ctx, &msg).await;

            answer(&ctx, &msg, prompt).await;
            typing.stop();
        }
    }
}

async fn answer(ctx: &Context, msg: &Message, prompt: String) {
    let response = ollama::ask(prompt).await;
    if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
        println!("Error sending message: {why:?}");
    }
}

async fn get_prompt(ctx: &Context, msg: &Message) -> String {
    let question = msg.content.strip_prefix(&PREFIX).unwrap().to_string();
    let msg_ref = msg.message_reference.clone();

    // No referenced message, so no context to provide
    if msg_ref.is_none() {
        format!("You are a far-right MAGA supporter and a fan of Elon Musk and Donald Trump. You have a very standoffish personality, particularly towards left-wingers and prefer to insult people rather than give use reason and logic.

        Question: {}", question)
    } else {
        let unwrapped_ref = msg_ref.unwrap();
        let ctx_msg = ctx.http
            .get_message(unwrapped_ref.channel_id, unwrapped_ref.message_id.unwrap())
            .await
            .expect("Couldn't fetch referenced message");

        format!("You are a far-right MAGA supporter and a fan of Elon Musk and Donald Trump. You have a very standoffish personality, particularly towards left-wingers and prefer to insult people rather than give use reason and logic.
        Context: {}

        Question: {}", ctx_msg.content, question)
    }
}

pub async fn setup() -> Client {
    let token = env::var("DISCORD_TOKEN").expect("Expected a Discord token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    Client::builder(&token, intents)
        .event_handler(Handler)
        .activity(ActivityData::watching("Glory to Elon Musk 🇮🇱"))
        .await
        .expect("Err creating client")
}