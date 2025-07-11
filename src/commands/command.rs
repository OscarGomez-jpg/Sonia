use async_trait::async_trait;
use serenity::all::{Context, Message};
use std::collections::HashMap;

use crate::commands;

pub trait Command: Send + Sync {
    fn name(&self) -> &'static str;
}

#[async_trait]
pub trait ExecutableCommand: Command {
    async fn execute(&self, ctx: &Context, msg: &Message);
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
        let re = regex::Regex::new(r"^>\w+\s").unwrap();

        if re.is_match(msg.content.as_str()) {
            let splitted_command = msg.content.split_whitespace().next().unwrap_or("");
            // println!("{:?}, msg.content: {:?}", msg.author.name, msg.content);
            if let Some(command) = self.commands.get(splitted_command) {
                command.execute(ctx, msg).await;
            }
        } else {
            return;
        }
    }
}
