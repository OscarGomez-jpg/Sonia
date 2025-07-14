use serenity::all::standard::macros::command;

pub struct LeaveCommand;

impl crate::commands::command::Command for LeaveCommand {
    fn name(&self) -> &'static str {
        ">leave"
    }
}

#[async_trait::async_trait]
impl crate::commands::command::ExecutableCommand for LeaveCommand {
    async fn execute(
        &self,
        ctx: &serenity::prelude::Context,
        msg: &serenity::model::prelude::Message,
    ) {
        let guild_id = msg.guild_id.unwrap();

        let manager = songbird::get(ctx)
            .await
            .expect("Songbird Voice client is not initialized")
            .clone();

        let has_handler = manager.get(guild_id).is_some();

        if has_handler {
            if let Err(e) = manager.remove(guild_id).await {
                msg.channel_id
                    .say(&ctx.http, format!("Error leaving the voice channel: {}", e))
                    .await
                    .unwrap();
            }

            msg.channel_id
                .say(&ctx.http, "Left the voice channel")
                .await
                .unwrap();
        } else {
            msg.channel_id
                .say(&ctx.http, "Not in a channel")
                .await
                .unwrap();
        }
    }
}
