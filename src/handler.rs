use rand::{rngs::StdRng, Rng, SeedableRng};
use serenity::{
    all::{CreateAttachment, CreateMessage, EventHandler, Message},
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

            let attachment = CreateAttachment::url(&ctx.http, &meme_url).await;

            match attachment {
                Ok(meme) => {
                    let builder = CreateMessage::new().content("This is your meme :)");
                    let meme_msg = msg
                        .channel_id
                        .send_files(&ctx.http, vec![meme], builder)
                        .await;

                    match meme_msg {
                        Ok(_) => {}
                        Err(why) => {
                            println!("{why:?}");
                        }
                    }
                }

                Err(why) => {
                    let error_img_url = "https://res.cloudinary.com/dyegt26ww/image/upload/v1718390339/base_sonia_hbg5s4.png";
                    let error_attachment_res =
                        CreateAttachment::url(&ctx.http, error_img_url).await;

                    match error_attachment_res {
                        Ok(err_attachment) => {
                            let builder =
                                CreateMessage::new().content("Could not find any meme :(");
                            let err_msg = msg
                                .channel_id
                                .send_files(&ctx.http, vec![err_attachment], builder)
                                .await;

                            match err_msg {
                                Ok(_) => {}
                                Err(why) => {
                                    println!("{why:?}");
                                }
                            }
                        }

                        Err(send_error) => {
                            println!("{send_error:?}");
                            if let Err(err_msg_err) = msg
                                .channel_id
                                .say(&ctx.http, "No se pudo encontrar un meme :(")
                                .await
                            {
                                println!("Error sending the meme: {err_msg_err:?}");
                            }
                        }
                    }

                    println!("Fatal error sending the meme: {why:?}");
                }
            }
        }
    }
}
