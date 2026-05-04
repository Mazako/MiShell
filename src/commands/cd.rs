use std::{env::args, path::PathBuf};

use crate::{command::Command, command_type::CommandType, commands::is_executable, shell_state::ShellState};

pub struct Cd {
    args: Vec<String>,
}

impl Cd {
    pub(super) fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl Command for Cd {
    fn execute(&self, ctx: &mut ShellState) {
        if self.args.is_empty() {
            return;
        }
        let dir = PathBuf::from(&self.args[0]);
        let normalized_dir = if dir.starts_with("/") {
            dir
        } else {
            dir
        };
        if !normalized_dir.exists() {
            println!("cd: {}: No such file or directory", normalized_dir.to_str().unwrap());
            return;
        }
        if normalized_dir.is_file() {
            println!("cd: not a directory: {}", normalized_dir.to_str().unwrap());
            return;
        }
        if let Ok(metadata) = normalized_dir.metadata() {
            if is_executable(metadata) {
                ctx.cwd = normalized_dir;
                println!()
            } else {
                println!("permission denied");
            }
        } else {
            panic!()
        }
    }

    fn args(&self) -> Vec<String> {
        self.args.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "cd"
    }
}