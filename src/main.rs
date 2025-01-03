use dotenv::dotenv;
use std::env;

use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let user_id = env::vars()
            .find(|(key, _)| *key == "USER_ID")
            .expect("Couldn't find the USER_ID enviromental variable")
            .1;

        let user_id = user_id
            .parse()
            .expect("Couldn't parse the USER_ID enviromental variable into a number");

        let user = UserId::new(user_id);

        let builder = CreateMessage::new().content("Hello from rust");
        user.direct_message(&ctx, builder).await.unwrap();

        std::process::exit(0);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::vars()
        .find(|(key, _)| *key == "DISCORD_TOKEN")
        .expect("Couldn't find the DISCORD_TOKEN enviromental variable")
        .1;

    let intents = GatewayIntents::DIRECT_MESSAGES;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
