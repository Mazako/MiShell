use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use crate::command::Command;
use crate::command_type::CommandType;

pub struct Type {
    pub(super) args: Vec<String>,
    pub(super) cmd_cache: Arc<HashMap<String, PathBuf>>,
}

impl Type {
    pub(super) fn new(args: Vec<String>, cmd_cache: Arc<HashMap<String, PathBuf>>) -> Self {
        Self { args, cmd_cache }
    }
}

impl Command for Type {
    fn execute(&self) {
        let args = self.args();
        if args.is_empty() {
            println!();
            return;
        }
        let cmd = super::command_from_input(&args[0], &self.cmd_cache);
        match cmd.command_type() {
            CommandType::Builtin => println!("{} is a shell builtin", cmd.name()),
            CommandType::Executable(path) => println!("{} is {}", cmd.name(), path.to_str().unwrap()),
            CommandType::Unrecognized => println!("{}: not found", cmd.name()),
        }
    }

    fn args(&self) -> Vec<String> {
        self.args.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "type"
    }
}
