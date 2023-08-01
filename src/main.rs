use anyhow::anyhow;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::application::command::CommandOptionType;
use serenity::{async_trait, model::prelude::GuildId};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

mod commands;
use commands::{valorant::*};

struct Bot; 

#[async_trait]
impl EventHandler for Bot {

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(702085427682869350);

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| { command.name("hello").description("Say hello") });
            commands.create_application_command(|command| { 
                command
                    .name("getuserbyname")
                    .description("Get your Valorant user by username")
                    .create_option(|option| {
                        option
                            .name("region")
                            .description("The region of the user")
                            .kind(CommandOptionType::String)
                            .required(true)
                            .add_string_choice("NA", "na1")
                            .add_string_choice("EUW", "euw1")
                            .add_string_choice("EUN", "eun1")
                            .add_string_choice("KR", "kr")
                            .add_string_choice("JP", "jp1")
                            .add_string_choice("BR", "br1")
                            
                    })
                    .create_option(|option| {
                        option
                            .name("username")
                            .description("The username of the user")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })
            })
        }).await.unwrap();

        info!("{:?}", commands)
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let response_content = match command.data.name.as_str() {
                "hello" => "hello".to_owned(),
                "getuserbyname" => {
                    // Get the username and region from the command
                    let username = command.data.options[0].value.as_ref().unwrap().as_str().unwrap().to_owned();
                    let region = command.data.options[1].value.as_ref().unwrap().as_str().unwrap().to_owned();

                    // Get the user from the Riot API
                    let user = get_user_by_name(username, region).await;
                    user.to_owned();
                }

                command => unreachable!("Unexpected command: {}", command),
            };

            let create_interaction_response = command.create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(response_content))
            });

            if let Err(why) = create_interaction_response.await {
                eprintln!("Cannot respond to slash command: {}", why)
            }
        }
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}