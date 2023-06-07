use std::env;

// Serenity
use serenity::{
    async_trait,
};
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

mod commands;
use commands::{fun::*, valorant::*};
struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[group]
#[prefixes("general")]
#[commands(ping, sqrt)]
struct General;

#[group]
#[prefixes("fun")]
#[commands(cat, shiba)]
struct Fun;

#[group]
#[prefixes("valorant")]
#[commands(getuser)]
struct Valorant;

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("=="))
        .group(&GENERAL_GROUP)
        .group(&FUN_GROUP)
        .group(&VALORANT_GROUP);
    
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
    let args = msg.content.split_whitespace().collect::<Vec<&str>>();
    let number = args[1].parse::<f64>().unwrap();


    assert!(number >= 0.0, "number must be positive"); 
    let sqrt = number.sqrt();
    msg.reply(ctx, format!("The square root of {} is {}", number, sqrt)).await?;
    
    Ok(())
}
