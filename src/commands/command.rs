use async_trait::async_trait;
use serenity::all::{Context, Message};
use std::collections::HashMap;

pub trait Command: Send + Sync {
    fn name(&self) -> &'static str;
}

#[async_trait]
pub trait ExecutableCommand: Command {
    async fn execute<'a>(&'a self, ctx: &'a Context, msg: &'a Message);
}

pub struct CommandManager {
    commands: HashMap<&'static str, Box<dyn ExecutableCommand>>,
}

impl CommandManager {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register_command(&mut self, command: Box<dyn ExecutableCommand>) {
        self.commands.insert(command.name(), command);
    }

    pub async fn handle_message(&self, ctx: &Context, msg: &Message) {
        if let Some(command) = self.commands.get(msg.content.as_str()) {
            command.execute(ctx, msg).await;
        }
    }
}
