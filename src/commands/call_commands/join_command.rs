use serenity::all::Context;
use songbird::{input::Input, tracks::Track, TrackEvent};

pub struct JoinCommand;

impl crate::commands::command::Command for JoinCommand {
    fn name(&self) -> &'static str {
        ">join"
    }
}

// impl JoinCommand {
//     async fn join_and_speak(
//         manager: &songbird::Songbird,
//         _ctx: &Context,
//         guild_id: serenity::model::id::GuildId,
//         channel_id: serenity::model::id::ChannelId,
//         text: &str,
//     ) {
//         // // Generate WAV file with Festival
// println!("Iniciando text2wave...");
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
//
// // Join voice channel and play audio
// println!(
//     "From guild_id: {:?} and channel_id: {:?}",
//     guild_id, channel_id
// );

// if let Ok(_call) = join_result {
//     //TODO:See waht to do with the call
//
//     // if let Some(handler_lock) = manager.get(guild_id) {
//     //     println!("Joined voice channel successfully!");
//     //     let mut handler = handler_lock.lock().await;
//     //     let source = songbird::input::File::new("output.wav");
//     //     handler.play(Track::from(source));
//     //     println!("Playing audio in the voice channel...");
//     // } else {
//     //     println!("Failed to get handler lock for the voice channel.");
//     // }
// } else {
//     println!("Error joining the channel: {:?}", join_result);
// }
//     }
// }

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

// fn check_msg(result: SerenityResult<Message>) {
//     if let Err(why) = result {
//         println!("Error sending message: {:?}", why);
//     }
// }
