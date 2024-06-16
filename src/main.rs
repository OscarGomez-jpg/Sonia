mod fetcher;
mod handler;

use dotenv::dotenv;
use fetcher::fetch_memes;
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

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating user");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

    fetch_memes().await.unwrap();
}
