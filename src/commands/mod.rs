mod echo;
mod exec;
mod exit;
mod type_cmd;
mod unknown;
mod pwd;
mod cd;

use std::fs::{self, Metadata};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use crate::command::Command;
use crate::commands::exec::Exec;
use crate::commands::pwd::Pwd;
use crate::shell_state::ShellState;

pub use cd::Cd;
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

fn parse_args(args: &str) -> Vec<String> {
    let mut normal: Vec<String> = vec![];
    let mut current_token: String = "".into();
    let mut quote_mode = false;
    for ele in args.trim().chars() {
        if ele == ' ' {
            if quote_mode {
                current_token.push(ele);
            } else {
                if !current_token.is_empty() {
                    normal.push(current_token);
                    current_token = "".into();
                }
            }
        } else if ele == '\'' {
            quote_mode = !quote_mode;
        } else {
            current_token.push(ele);
        }
    }
    if !current_token.trim().is_empty() {
        normal.push(current_token);
    }
    normal
}

pub fn command_from_input(input: &str, ctx: &ShellState) -> Box<dyn Command> {
    let (command, args) = command_args(input);
    match command {
        "echo" => Box::new(Echo::new(parse_args(args))),
        "exit" => Box::new(Exit),
        "type" => Box::new(Type::new(parse_args(args))),
        "pwd" => Box::new(Pwd),
        "cd" => Box::new(Cd::new(parse_args(args))),
        _ => {
            if let Some(path) = find_in_path(command, &ctx.path_dirs) {
                Box::new(Exec::new(command.to_string(), path, parse_args(args)))
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
