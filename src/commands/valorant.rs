use serenity::prelude::*;
use serenity::model::channel::Message;

use serenity::framework::standard::macros::{command};
use serenity::framework::standard::{CommandResult};

use reqwest::{self, Url};

#[command]
#[description = "Get user by usrname and region"]
async fn getuser(ctx: &Context, msg: &Message) -> CommandResult {

    // Get params from message
    let params = msg.content.split(" ").collect::<Vec<&str>>();

    // Check if params are valid
    // 0: prefix
    // 1: command
    // 2: username
    // 3: region
    if params.len() < 3 {
        msg.reply(ctx, "Invalid params").await?;
        return Ok(());
    }

    // Get username
    let username = params[2];

    // Get region
    let region = params[3];

    // parse region to region code
    let region_code = match region {
        "na" => "na1",
        "eu" => "euw1",
        "kr" => "kr",
        "jp" => "jp1",
        "br" => "br1",
        "oc" => "oc1",
        "eune" => "eun1",
        "tr" => "tr1",
        "ru" => "ru",
        "la" => "la1",
        "la2" => "la2",

        _ => {
            msg.reply(ctx, "Invalid region").await?;
            return Ok(());
        }
    };

    // create string for url
    let url = format!("https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-name/{}", region_code, username);

    let url = Url::parse(&url).unwrap();
    let res = reqwest::get(url).await?;
    println!("{:?}", res.status());

    Ok(())
}

