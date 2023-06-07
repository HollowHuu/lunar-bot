use serenity::prelude::*;
use serenity::model::channel::Message;

use serenity::framework::standard::macros::{command};
use serenity::framework::standard::{CommandResult};

use reqwest::{self, Url};

#[command]
#[description = "Find some random cat pictures"]
async fn cat(ctx: &Context, msg: &Message) -> CommandResult {

    let url = Url::parse("http://shibe.online/api/cats").unwrap();
    let res = reqwest::get(url).await?;

    if res.status() != 200 {
        msg.reply(ctx, "Failed to get cat picture").await?;
        return Ok(());
    }

    let response: Vec<String> = res.json().await?;




    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Cat Picture");
            e.image(&response[0]);
            e
        })
    }).await?;


    Ok(())
}

#[command]
#[description = "Find some random shiba pictures"]
async fn shiba(ctx: &Context, msg: &Message) -> CommandResult {

    let url = Url::parse("http://shibe.online/api/shibes").unwrap();
    let res = reqwest::get(url).await?;

    if res.status() != 200 {
        msg.reply(ctx, "Failed to get shiba picture").await?;
        return Ok(());
    }

    let response: Vec<String> = res.json().await?;




    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Shiba Picture");
            e.image(&response[0]);
            e
        })
    }).await?;


    Ok(())
}