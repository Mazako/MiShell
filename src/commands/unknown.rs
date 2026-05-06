use std::io::Write;

use crate::command::{Command, Token};
use crate::command_type::CommandType;
use crate::shell_state::ShellState;

pub struct UnknownCommand {
    pub(super) command: String,
    pub(super) tokens: Vec<Token>,
}

impl UnknownCommand {
    pub(super) fn new(command: String, tokens: Vec<Token>) -> Self {
        Self { command, tokens }
    }
}

impl Command for UnknownCommand {
    fn execute(&self, _ctx: &mut ShellState) {
        let output = format!("{}: command not found", self.command);
        if let Some(redirect) = self.stderr() {
            let mut file = redirect.open_write();
            writeln!(file, "{output}").unwrap();
        } else {
            eprintln!("{output}");
        }
    }

    fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Unrecognized
    }

    fn name(&self) -> &str {
        &self.command
    }
}
