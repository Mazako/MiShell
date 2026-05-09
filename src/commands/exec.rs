use std::path::PathBuf;

use crate::command::Command;
use crate::command_type::CommandType;
use crate::shell_state::ShellState;
use crate::token::Input;

pub struct Exec {
    input: Input,
    path: PathBuf,
}

impl Exec {
    pub(super) fn new(input: Input, path: PathBuf) -> Self {
        Self { input, path }
    }
}

impl Command for Exec {
    fn execute(&self, _ctx: &mut ShellState) {
        let mut command = std::process::Command::new(&self.input.command);
        command.args(self.args());
        self.apply_redirects(&mut command);
        command.status().unwrap();
    }

    fn execute_background(&self) -> std::process::Child {
        std::process::Command::new(&self.input.command)
            .args(&self.input.args)
            .spawn()
            .unwrap()
    }

    fn input(&self) -> Input {
        self.input.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Executable(PathBuf::from(&self.path))
    }

    fn name(&self) -> &str {
        &self.input.command
    }
}
