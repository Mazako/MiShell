use std::path::PathBuf;
use std::process::Stdio;

use crate::command::Token;
use crate::command::{Command, StreamTarget};
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
        match self.stdout() {
            StreamTarget::Redirect(redirect) => {
                let file = redirect.open_write();
                command.stdout(Stdio::from(file));
            }
            StreamTarget::Inherit => {}
        }
        match self.stderr() {
            StreamTarget::Redirect(redirect) => {
                let file = redirect.open_write();
                command.stderr(Stdio::from(file));
            }
            StreamTarget::Inherit => {}
        }
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
