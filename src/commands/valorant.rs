use anyhow::Ok;
use serenity::model::channel::Message;
use serenity::prelude::*;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: String,
    accountId: String,
    puuid: String,
    name: String,
    profileIconId: i32,
    revisionDate: i64,
    summonerLevel: i32
}


pub async fn get_user_by_name(name: String, region: String) -> serenity::builder::CreateEmbed{
    // control variables
    let string = format!("https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-name/{}", &region, &name);

    // get the user from riot api
    let client = Client::new();
    let res = client.get(string)
        .header("X-Riot-Token", "TOKEN")
        .send()
        .await
        .unwrap();

    let status = &res.status();
    assert_eq!(status, &StatusCode::OK);

    let body = res.text().await.unwrap();
    let user = serde_json::from_str::<User>(&body).unwrap();

    // create the embed
    let mut embed = serenity::builder::CreateEmbed::default();
    let embed = embed.title(format!("{}'s Profile", &user.name))
        .description(format!("Level: {}", &user.summonerLevel))
        .field("Region", &region, true)
        .field("ID", &user.id, true)
        .field("Account ID", &user.accountId, true)
        .field("PUUID", &user.puuid, true)
        .field("Profile Icon ID", &user.profileIconId, true)
        .field("Revision Date", &user.revisionDate, true);

    // Return the embed
    return embed.clone()

}