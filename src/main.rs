use dotenv::dotenv;
use std::env;

use libparted::Device;
use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::prelude::*;
use std::process::Command;

const DISK_SCRIPT: &str = include_str!("./smartcheck.sh");

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let user = get_user_from_env().expect("Couldn't get user from env");
        let disk_reports = get_disk_report();

        for report in disk_reports {
            let builder = CreateMessage::new().content(report);
            user.direct_message(&ctx, builder).await.unwrap();
        }

        std::process::exit(0);
    }
}

fn get_user_from_env() -> Option<UserId> {
    let user_id = env::vars().find(|(key, _)| *key == "USER_ID")?.1;
    let user_id = user_id.parse().ok()?;

    Some(UserId::new(user_id))
}

//TODO: Refactor this to pass in disks detected from libparted
fn get_disk_report() -> Vec<String> {
    let disks: Vec<String> = Device::devices(true)
        .map(|device| device.path().to_str().unwrap().to_string())
        .collect();
    let mut disk_reports = Vec::new();

    for disk in disks {
        let output = Command::new("bash")
            .arg("-c")
            .arg(DISK_SCRIPT)
            .arg(disk.clone())
            .output()
            .expect("Failed to execute script");

        if output.status.success() {
            disk_reports.push(String::from_utf8_lossy(&output.stdout).to_string());
        } else {
            disk_reports.push(format!(
                "{} Failed to run disk check script {}",
                disk,
                String::from_utf8_lossy(&output.stdout)
            ));
        }
    }

    disk_reports
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
