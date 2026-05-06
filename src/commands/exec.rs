use std::path::PathBuf;

use crate::command::{Command, Token};
use crate::command_type::CommandType;
use crate::shell_state::ShellState;

pub struct Exec {
    name: String,
    path: PathBuf,
    tokens: Vec<Token>,
}

impl Exec {
    pub(super) fn new(name: String, path: PathBuf, tokens: Vec<Token>) -> Self {
        Self { name, path, tokens }
    }
}

impl Command for Exec {
    fn execute(&self, _ctx: &mut ShellState) {
        let mut command = std::process::Command::new(&self.name);
        command.args(self.args());
        self.apply_redirects(&mut command);
        command.status().unwrap();
    }

    fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Executable(self.path.clone())
    }

    fn name(&self) -> &str {
        &self.name
    }
}
