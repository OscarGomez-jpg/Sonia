use serenity::all::{Context, Message};

use crate::commands::command::{Command, ExecutableCommand};

pub struct PingCommand;

impl Command for PingCommand {
    fn name(&self) -> &'static str {
        ">ping"
    }
}

#[async_trait::async_trait]
impl ExecutableCommand for PingCommand {
    async fn execute(&self, ctx: &Context, msg: &Message) {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Pong<").await {
            println!("Error sending message: {why:?}");
        }
    }
}
