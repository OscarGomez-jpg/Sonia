mod fetcher;
mod handler;
mod urls_manager;

use dotenv::dotenv;
use handler::Handler;
use serenity::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler::new().await;
    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating user");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
