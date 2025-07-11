use std::sync::Arc;

use serenity::{
    all::{Context, CreateAttachment, CreateMessage, Message},
    futures::lock::Mutex,
};

use crate::commands::command::{Command, ExecutableCommand};

use super::urls_manager::UrlManager;

pub struct MemeCommand {
    pub url_manager: Arc<Mutex<UrlManager>>,
}

impl Command for MemeCommand {
    fn name(&self) -> &'static str {
        ">meme"
    }
}

#[async_trait::async_trait]
impl ExecutableCommand for MemeCommand {
    async fn execute(&self, ctx: &Context, msg: &Message) {
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

                meme_msg.expect("Panic sending the meme");
            }

            Err(why) => {
                let error_img_url = "https://res.cloudinary.com/dyegt26ww/image/upload/v1718390339/base_sonia_hbg5s4.png";
                let error_attachment_res = CreateAttachment::url(&ctx.http, error_img_url).await;

                match error_attachment_res {
                    Ok(err_attachment) => {
                        let builder = CreateMessage::new().content("Could not find any meme :(");
                        let err_msg = msg
                            .channel_id
                            .send_files(&ctx.http, vec![err_attachment], builder)
                            .await;

                        err_msg.expect("Panic sending the error message");
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
