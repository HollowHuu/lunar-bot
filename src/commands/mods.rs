use serenity::model::{id::ChannelId, channel::GuildChannel};
use serenity::client::Context;
use serde::{Deserialize, Serialize, Serializer};
use serenity::model::prelude::PermissionOverwrite;

#[derive(Serialize, Deserialize)]
struct ChannelOptions {
    name: String,
    position: Option<i64>,
    parent_id: Option<u64>,
    permission_overwrites: Option<Vec<PermissionOverwrite>>,
    nsfw: bool,
}

pub async fn nuke(channel_id: ChannelId, ctx: &Context) -> bool {

    // Delete the channel and create a new one with the same name and position as the old one.
    let channel = channel_id.to_channel(&ctx.http).await.unwrap();
    
    // Guild
    let guild_id = channel.guild().unwrap().guild_id;
    
    let serializer = Serializer::new();
    let serialized_channel = channel.clone().serialize(Serializer).unwrap();

    // Json options
    let options: ChannelOptions = ChannelOptions {
        name: name,
        position: channel.position(),
        parent_id: channel.category().unwrap().id(),
        permission_overwrites: channel.permission_overwrites,
        nsfw: channel.is_nsfw(),
    };

    print!("{:?}", options);

    // &ctx.http.create_channel(guild_id, ).await.unwrap();

    // let messages = channel_id.messages(&ctx.http, |retriever| {
    //     retriever.limit(100)
    // }).await.unwrap();

    // for message in messages {
    //     message.delete(&ctx.http).await.unwrap();
    // }

    true
}