use std::env;

// use anyhow::Ok;
use serenity::model::{
    id::UserId,
    prelude::User,
    guild
};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json;
use serenity::builder::CreateEmbed;
use tracing::log::info;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Profile {
    success: bool,
    puuid: String,
    gameName: String,
    tagLine: String,
}


pub async fn profile(user_id: UserId) -> Result<CreateEmbed, Box<dyn std::error::Error + Send + Sync>> {
    let api_string = format!("https://valorant.aesirdev.tech/api/bot/profile");
    let api_key = env::var("AESIR_API_KEY").unwrap();
    info!("user_id: {}", user_id);
    info!("API Key: {}", api_key);
    
    let client = Client::new();
    let res = client.post(api_string)
        .header("Content-Type", "application/json")
        .header("Authorization", "TEST")
        .body(format!("{{\"user_id\": \"{}\"}}", user_id))
        .send()
        .await?;

    let status = &res.status();
    assert_eq!(status, &StatusCode::OK);

    let body = res.text().await.unwrap();
    info!("Body: {}", body);
    let profile = serde_json::from_str::<Profile>(&body).unwrap();

    let mut embed = CreateEmbed::default();
    embed.title(format!("{}#{}", &profile.gameName, &profile.tagLine));
    embed.description(format!("PUUID: {}", &profile.puuid));

    Ok(embed)

}

pub async fn update_rank(guild_id: i64) {
    // Fetch all users from the guild
    // For each user, get their profile
    // Update their rank
    guild::Guild::members()

}