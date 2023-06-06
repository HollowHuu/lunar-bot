use std::env;

// Serenity
use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[group]
#[commands(ping, sqrt)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("=="))
        .group(&GENERAL_GROUP);
    
    let token = env::var("token").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await        
        .expect("Error creating client");

    // Start listening for events by starting a single shard.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn sqrt(ctx: &Context, msg: &Message) -> CommandResult {
    // Get the square root of a number
    print!("Message: {}", msg.content);
    let number: f64 = msg.content.parse().unwrap();

    // assert!(number.is_err()); 
    let sqrt = number.sqrt();
    msg.reply(ctx, format!("The square root of {} is {}", number, sqrt)).await?;
    
    Ok(())
}