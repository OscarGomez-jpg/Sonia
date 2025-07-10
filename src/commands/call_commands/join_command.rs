use crate::commands::command::Command;

pub struct JoinCommand;

impl Command for JoinCommand {
    fn name(&self) -> &'static str {
        ">join"
    }
}
