use crate::commands::command::Command;

pub struct TalkCommand;

impl Command for TalkCommand {
    fn name(&self) -> &'static str {
        ">talk"
    }
}
