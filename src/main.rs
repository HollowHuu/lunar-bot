use serenity::{
    async_trait,
    model::{
        // channel::Message, 
        gateway::Ready,
        application::{
            interaction::{Interaction, InteractionResponseType},
            // command::CommandOptionType
        },
        prelude::GuildId
    },
    prelude::*
};

mod commands;
use commands::mods::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(1110180774763827261);

        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| { command.name("hello").description("Say hello") });
            commands.create_application_command(|command| { command.name("nuke").description("Nuke the last 100 messages in the channel") })
        }).await.unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {

                // Hello command
                "hello" => {
                    let _ = command.create_interaction_response(&ctx.http, |response| {
                        response.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.content("Hello, world!")
                            })
                    }).await;
                }

                // Nuke Command
                "nuke" => {
                    let _ = command.create_interaction_response(&ctx.http, |response| {
                        response.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.content("Nuking channel...")
                            })
                    }).await;

                    let channel_id = command.channel_id;
                    nuke(channel_id, &ctx).await;
                }

                // Unknown command
                comamnd => {
                    let _ = command.create_interaction_response(&ctx.http, |response| {
                        response.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.content(format!("Unknown command: {}", comamnd))
                            })
                    }).await;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let application_id: u64 = std::env::var("APPLICATION_ID")
        .expect("Expected an Application Id in the environment")
        .parse()
        .expect("Application Id must be a valid u64");
    
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

}