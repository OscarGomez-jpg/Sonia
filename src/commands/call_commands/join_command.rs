use serenity::all::Context;
use serenity::prelude::*;
use songbird::{
    id::{ChannelId, GuildId},
    tracks::Track,
};
use std::process::Command;

pub struct JoinCommand;

impl crate::commands::command::Command for JoinCommand {
    fn name(&self) -> &'static str {
        ">join"
    }
}

impl JoinCommand {
    async fn join_and_speak(ctx: &Context, guild_id: GuildId, channel_id: ChannelId, text: &str) {
        // Generate WAV file with Festival
        println!("Iniciando text2wave...");
        let mut child = Command::new("text2wave")
            .arg("-o")
            .arg("output.wav")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .expect("No se pudo iniciar text2wave");

        if let Some(stdin) = child.stdin.as_mut() {
            use std::io::Write;
            println!("Escribiendo texto en stdin de text2wave...");
            stdin
                .write_all(text.as_bytes())
                .expect("No se pudo escribir en stdin");
        } else {
            println!("No se pudo abrir stdin para text2wave");
        }

        let status = child.wait().expect("No se pudo esperar a text2wave");
        println!("text2wave terminó con status: {:?}", status);

        // Verificar si el archivo se creó
        let wav_exists = std::path::Path::new("output.wav").exists();
        println!("¿Existe output.wav?: {}", wav_exists);

        // Join voice channel and play audio
        let manager = songbird::get(ctx).await;

        if let Some(manager) = manager {
            println!(
                "From guild_id: {:?} and channel_id: {:?}",
                guild_id, channel_id
            );

            let join_result = manager.join(guild_id, channel_id).await;
            println!("Intentando unirse al canal de voz...");

            if let Ok(handler_lock) = join_result {
                println!("Joined voice channel successfully!");
                let mut handler = handler_lock.lock().await;
                let source = songbird::input::File::new("output.wav");
                handler.play(Track::from(source).volume(0.5));
            } else {
                println!("Error al unirse al canal de voz: {:?}", join_result);
            }
        } else {
            println!("No se pudo obtener el manager de Songbird");
            return;
        }
    }
}

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

        JoinCommand::join_and_speak(
            ctx,
            msg.guild_id.unwrap().into(),
            msg.channel_id.into(),
            "Hello, I have joined the call!",
        )
        .await;
    }
}
