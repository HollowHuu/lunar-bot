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

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(702085427682869350);

        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| { command.name("hello").description("Say hello") })
        }).await.unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "hello" => {
                    let _ = command.create_interaction_response(&ctx.http, |response| {
                        response.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.content("Hello, world!")
                            })
                    }).await;
                }
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
    let token = "NjkyNjUxNTA4MTEzODY2ODIz.GLfBlz.z-WhFf-NfK-TcyelKSU0el9oDu8odSJNNgAigc";

    let application_id: u64 = 702085427682869350;

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