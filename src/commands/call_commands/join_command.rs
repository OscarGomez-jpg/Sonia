pub struct JoinCommand;

impl crate::commands::command::Command for JoinCommand {
    fn name(&self) -> &'static str {
        ">join"
    }
}

// let mut child = std::process::Command::new("text2wave")
//     .arg("-o")
//     .arg("output.wav")
//     .stdin(std::process::Stdio::piped())
//     .spawn()
//     .expect("Could not start text2wave process");
//
// if let Some(stdin) = child.stdin.as_mut() {
//     use std::io::Write;
//     stdin
//         .write_all(text.as_bytes())
//         .expect("Could not write to text2wave stdin");
// } else {
//     println!("Could not get stdin for text2wave");
// }
//
// let status = child
//     .wait()
//     .expect("Could not wait for text2wave to finish");
// println!("text2wave ended with status: {:?}", status);
//
// // Checks if the wav file was created
// let wav_exists = std::path::Path::new("output.wav").exists();
// println!("¿output.wav exists?: {}", wav_exists);

#[async_trait::async_trait]
impl crate::commands::command::ExecutableCommand for JoinCommand {
    async fn execute(
        &self,
        ctx: &serenity::prelude::Context,
        msg: &serenity::model::channel::Message,
    ) {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Joining the call...").await {
            println!("Error sending message: {why:?}");
        }

        let (guild_id, channel_id) = {
            let guild = msg.guild(&ctx.cache).unwrap();
            let channel_id = guild
                .voice_states
                .get(&msg.author.id)
                .and_then(|voice_state| voice_state.channel_id);

            (guild.id, channel_id)
        };

        //TODO:Handle The case where the user is not in a voice channel

        let manager = songbird::get(ctx)
            .await
            .expect("Songbird Voice client is not initialized")
            .clone();

        if let Ok(handler_lock) = manager.join(guild_id, channel_id.unwrap()).await {
            let _handler = handler_lock.lock().await;
        }
    }
}
