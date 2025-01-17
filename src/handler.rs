use std::sync::Arc;

use async_trait::async_trait;

use serenity::{
    all::{EventHandler, Message},
    prelude::*,
};

use crate::{
    command::CommandManager,
    commands::{meme_command::MemeCommand, ping_command::PingCommand},
    urls_manager::UrlManager,
};

pub struct Handler {
    command_manager: CommandManager,
}

impl Handler {
    pub async fn new() -> Self {
        let mut command_manager = CommandManager::new();
        let url_manager = Arc::new(serenity::futures::lock::Mutex::new(UrlManager::new().await));

        command_manager.register_command(Box::new(PingCommand));
        command_manager.register_command(Box::new(MemeCommand { url_manager }));

        Self { command_manager }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        self.command_manager.handle_message(&ctx, &msg).await;
    }
}
