use std::{path::PathBuf};

use crate::{command::Command, command_type::CommandType};

pub struct Exec {
    name: String,
    path: PathBuf,
    args: Vec<String>,
}

impl Exec {
    pub(super) fn new(name: String, path: PathBuf, args: Vec<String>) -> Self {
        Self { name, path, args }
    }
}

impl Command for Exec {
    fn execute(&self) {
        todo!()
    }

    fn args(&self) -> Vec<String> {
        self.args.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Executable(self.path.clone())
    }

    fn name(&self) -> &str {
        &self.name

    }
}