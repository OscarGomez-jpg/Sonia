use serenity::all::{Context, Message};

pub struct VoiceCommand {
    pub target_words: Vec<String>,
}

impl VoiceCommand {
    pub fn new() -> Self {
        VoiceCommand {
            target_words: vec!["hello".to_string(), "world".to_string()],
        }
    }
}
