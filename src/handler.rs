use rand::{rngs::StdRng, Rng, SeedableRng};
use serenity::{
    all::{EventHandler, Message},
    async_trait,
    prelude::*,
};

use crate::fetcher::fetch_memes;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut rng = StdRng::from_entropy();
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sendind message: {why:?}");
            }
        } else if msg.content == "!meme" {
            let meme_url;

            if let Ok(memes) = fetch_memes().await {
                meme_url = memes[rng.gen_range(0..memes.len())].url.clone();
            } else {
                meme_url = "Meme not found".to_string();
            }

            if let Err(why) = msg.channel_id.say(&ctx.http, meme_url).await {
                println!("Error sending the memes: {why:?}");
            }
        }
    }
}
