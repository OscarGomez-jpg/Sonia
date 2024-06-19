use std::sync::Arc;

use serenity::{
    all::{CreateAttachment, CreateMessage, EventHandler, Message},
    async_trait,
    prelude::*,
};

use crate::urls_manager::UrlManager;

pub struct Handler {
    url_manager: Arc<Mutex<UrlManager>>,
}

impl Handler {
    pub async fn new() -> Self {
        let url_manager = UrlManager::new().await;

        Self {
            url_manager: Arc::new(Mutex::new(url_manager)),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        //For now is the same as before, have to store somewhere else to
        //be reused each time it is called, maybe a singleton, but don't know
        //how to do that

        if msg.content == ">ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong<").await {
                println!("Error sendind message: {why:?}");
            }
        } else if msg.content == ">meme" {
            let mut url_manager = self.url_manager.lock().await;
            let meme = url_manager.get_meme().await;
            let meme_url = meme.url;

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
                                .say(&ctx.http, "Could not find any meme :(")
                                .await
                            {
                                println!("Error sending the meme: {err_msg_err:?}");
                            }
                        }
                    }

                    println!("Fatal error sending the meme: {why:?}");
                }
            }

            url_manager.save_state().unwrap();
        }
    }
}
