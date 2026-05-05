mod echo;
mod exec;
mod exit;
mod type_cmd;
mod unknown;
mod pwd;
mod cd;

use std::env::args;
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


#[derive(Eq, Debug, PartialEq)]
enum ParseMode {
    Normal,
    Quoted,
    DoubleQuoted
}

fn parse_args(args: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current_token = String::new();
    let mut quote_mode = ParseMode::Normal;

    let mut chars = args.trim().chars().peekable();

    while let Some(ch) = chars.next() {
        match quote_mode {
            ParseMode::Normal => {
                if ch == ' ' {
                    if !current_token.is_empty() {
                        tokens.push(current_token);
                        current_token = String::new();
                    }
                } else if ch == '\'' {
                    quote_mode = ParseMode::Quoted;
                } else if ch == '"' {
                    quote_mode = ParseMode::DoubleQuoted;
                } else if ch == '\\' {
                    if let Some(next) = chars.next() {
                        current_token.push(next);
                    }
                } else {
                    current_token.push(ch);
                }
            }
            ParseMode::Quoted => {
                if ch == '\'' {
                    quote_mode = ParseMode::Normal;
                } else {
                    current_token.push(ch);
                }
            }
            ParseMode::DoubleQuoted => {
                if ch == '"' {
                    quote_mode = ParseMode::Normal;
                } else if ch == '\\' {
                    if let Some(&next) = chars.peek() {
                        if matches!(next, '"' | '\\' | '$' | '`' | '\n') {
                            current_token.push(chars.next().unwrap());
                        } else {
                        }
                    }
                } else {
                    current_token.push(ch);
                }
            }
        }
    }

    if !current_token.trim().is_empty() {
        tokens.push(current_token);
    }

    tokens
}

pub fn command_from_input(input: &str, ctx: &ShellState) -> Box<dyn Command> {
    let parsed = parse_args(input);
    let (command, args) = if parsed.is_empty() {
        ("".to_string(), vec!["".to_string()])
    } else {
        (parsed[0].to_string(), parsed[1..].to_vec())
    };
    match command.as_str() {
        "echo" => Box::new(Echo::new(args)),
        "exit" => Box::new(Exit),
        "type" => Box::new(Type::new(args)),
        "pwd" => Box::new(Pwd),
        "cd" => Box::new(Cd::new(args)),
        _ => {
            if let Some(path) = find_in_path(&command, &ctx.path_dirs) {
                Box::new(Exec::new(command.to_string(), path, args))
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
