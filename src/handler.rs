use std::sync::Arc;

use async_trait::async_trait;

use serenity::{
    all::{CreateAttachment, CreateMessage, EventHandler, Message},
    prelude::*,
};

use crate::{
    command::{Command, CommandManager, ExecutableCommand},
    urls_manager::UrlManager,
};

pub struct PingCommand;

impl Command for PingCommand {
    fn name(&self) -> &'static str {
        ">ping"
    }
}

#[async_trait]
impl ExecutableCommand for PingCommand {
    async fn execute<'a>(&'a self, ctx: &'a Context, msg: &'a Message) {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Pong<").await {
            println!("Error sending message: {why:?}");
        }
    }
}

pub struct MemeCommand {
    url_manager: Arc<Mutex<UrlManager>>,
}

#[async_trait::async_trait]
impl Command for MemeCommand {
    fn name(&self) -> &'static str {
        ">meme"
    }
}

#[async_trait::async_trait]
impl ExecutableCommand for MemeCommand {
    async fn execute<'a>(&'a self, ctx: &'a Context, msg: &'a Message) {
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
                let error_attachment_res = CreateAttachment::url(&ctx.http, error_img_url).await;

                match error_attachment_res {
                    Ok(err_attachment) => {
                        let builder = CreateMessage::new().content("Could not find any meme :(");
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

pub struct Handler {
    command_manager: CommandManager,
}

impl Handler {
    pub async fn new() -> Self {
        let mut command_manager = CommandManager::new();
        let url_manager = Arc::new(Mutex::new(UrlManager::new().await));

        command_manager.register_command(Box::new(PingCommand));
        command_manager.register_command(Box::new(MemeCommand {
            url_manager: url_manager.clone(),
        }));

        Self { command_manager }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        self.command_manager.handle_message(&ctx, &msg).await;
    }
}
