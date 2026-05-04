mod echo;
mod exec;
mod exit;
mod type_cmd;
mod unknown;
mod pwd;


use std::fs::{self, Metadata};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use crate::command::Command;
use crate::commands::exec::Exec;
use crate::commands::pwd::Pwd;
use crate::shell_state::ShellState;

pub use echo::Echo;
pub use exit::Exit;
pub use type_cmd::Type;
pub use unknown::UnknownCommand;

fn command_args(command: &str) -> (&str, &str) {
    let command = command.trim();
    if let Some(idx) = command.find(' ') {
        let args = &command[idx + 1..].trim();
        return (&command[..idx], args);
    }
    (command, "")
}

fn whitespace_args(args: &str) -> Vec<String> {
    args.split_whitespace().map(|s| s.to_string()).collect()
}

pub fn command_from_input(input: &str, ctx: &ShellState) -> Box<dyn Command> {
    let (command, args) = command_args(input);
    match command {
        "echo" => Box::new(Echo::new(args.to_string())),
        "exit" => Box::new(Exit),
        "type" => Box::new(Type::new(whitespace_args(args))),
        "pwd" => Box::new(Pwd),
        _ => {
            if let Some(path) = find_in_path(command, &ctx.path_dirs) {
                Box::new(Exec::new(command.to_string(), path, whitespace_args(args)))
            } else {
                Box::new(UnknownCommand::new(command.to_string()))
            }
        }
    }
}

pub fn find_in_path(command: &str, path_dirs: &[PathBuf]) -> Option<PathBuf> {
    for dir in path_dirs {
        if let Ok(res) = fs::read_dir(dir) {
            for e in res.flatten() {
                if let Ok(metadata) = e.metadata()
                    && metadata.is_file()
                    && let Ok(name) = e.file_name().into_string()
                    && name == command
                    && is_executable(metadata)
                {
                    return Some(e.path());
                }
            }
        }
    }
    None
}

fn is_executable(metadata: Metadata) -> bool {
    metadata.permissions().mode() & 0o111 != 0
}
