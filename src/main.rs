use dotenv::dotenv;
use std::env;

use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::prelude::*;
use std::process::Command;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        // let disks = get_system_disks();

        let builder = CreateMessage::new().content("Hello from rust");
        let user = get_user_from_env().expect("Couldn't get user from env");
        user.direct_message(&ctx, builder).await.unwrap();

        std::process::exit(0);
    }
}

fn get_user_from_env() -> Option<UserId> {
    let user_id = env::vars().find(|(key, _)| *key == "USER_ID")?.1;
    let user_id = user_id.parse().ok()?;

    Some(UserId::new(user_id))
}

//TODO: Refactor this to pass in disks detected from libparted
fn get_disk_health() -> String {
    let script = include_str!("./smartcheck.sh");

    let output = Command::new("bash")
        .arg("-c") // Pass the command string
        .arg(script)
        .output() // Capture the output
        .expect("Failed to execute script");

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        "Failed to run disk check script".to_string()
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::vars()
        .find(|(key, _)| *key == "DISCORD_TOKEN")
        .expect("Couldn't find the DISCORD_TOKEN enviromental variable")
        .1;

    println!("{}", get_disk_health());

    todo!();

    let intents = GatewayIntents::DIRECT_MESSAGES;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
