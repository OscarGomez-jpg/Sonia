use songbird::{
    input::{self},
    tracks::{Track, TrackHandle},
};

pub struct Test;

impl crate::commands::command::Command for Test {
    fn name(&self) -> &'static str {
        ">test"
    }
}

// fn open_audio_with_symphonia(
//     path: &str,
// ) -> Result<Box<dyn FormatReader + Send>, Box<dyn std::error::Error + Send>> {
//     // Open the file.
//     let file = File::open(path).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;
//     let mss = MediaSourceStream::new(Box::new(file), Default::default());
//
//     // Create a hint to help the format registry guess what format reader is appropriate.
//     let mut hint = Hint::new();
//     if let Some(ext) = path.split('.').last() {
//         hint.with_extension(ext);
//     }
//
//     // Probe the media source stream for a format.
//     let probed = get_probe()
//         .format(&hint, mss, &Default::default(), &Default::default())
//         .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;
//     let format = probed.format;
//
//     Ok(format)
// }

#[async_trait::async_trait]
impl crate::commands::command::ExecutableCommand for Test {
    async fn execute(
        &self,
        ctx: &serenity::prelude::Context,
        msg: &serenity::model::channel::Message,
    ) {
        let guild_id = msg.guild_id.unwrap();

        let manager = songbird::get(ctx)
            .await
            .expect("Expected songbird client to be initialized")
            .clone();

        if let Some(handler_lock) = manager.get(guild_id) {
            let mut handler = handler_lock.lock().await;
            let input = input::File::new("output.wav");
            let res: TrackHandle = handler.play(Track::from(input).volume(1.));
            dbg!(
                "Playing audio in the voice channel: {:?}",
                res.get_info().await.unwrap()
            );
        } else {
            dbg!("songbird client not found for guild_id: {:?}", guild_id);
        }
    }
}
