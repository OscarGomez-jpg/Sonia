use std::sync::Arc;

use async_trait::async_trait;

use serenity::{
    all::{EventHandler, Message},
    prelude::*,
};

use crate::commands::{
    call_commands::{join_command::JoinCommand, talk_command::TalkCommand},
    command::CommandManager,
    meme_commands::{meme_command::MemeCommand, urls_manager::UrlManager},
    ping_commands::ping_command::PingCommand,
};

pub struct Handler {
    command_manager: CommandManager,
    songbird: Arc<songbird::Songbird>,
}

impl Handler {
    pub async fn new(songbird: Arc<songbird::Songbird>) -> Self {
        let mut command_manager = CommandManager::new();
        let url_manager = Arc::new(serenity::futures::lock::Mutex::new(UrlManager::new().await));

        command_manager.register_command(Box::new(PingCommand));
        command_manager.register_command(Box::new(MemeCommand { url_manager }));
        command_manager.register_command(Box::new(TalkCommand));
        command_manager.register_command(Box::new(JoinCommand));

        Self {
            command_manager,
            songbird,
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        self.command_manager.handle_message(&ctx, &msg).await;
    }
}
