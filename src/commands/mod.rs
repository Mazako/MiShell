mod echo;
mod exit;
mod exec;
mod unknown;
mod type_cmd;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use crate::command::Command;

pub use echo::Echo;
pub use exit::Exit;
pub use unknown::UnknownCommand;
pub use type_cmd::Type;

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

pub fn command_from_input(
    input: &str,
    cmd_cache: &Arc<HashMap<String, PathBuf>>,
) -> Box<dyn Command> {
    let map: &HashMap<String, PathBuf> = cmd_cache.as_ref();
    let (command, args) = command_args(input);
    match command {
        "echo" => Box::new(Echo::new(args.to_string())),
        "exit" => Box::new(Exit),
        "type" => Box::new(Type::new(
            whitespace_args(args),
            Arc::clone(cmd_cache),
        )),
        _ => {
            if let Some(path) = map.get(command) {
                Box::new(exec::Exec::new(
                    command.to_string(),
                    path.clone(),
                    whitespace_args(args),
                ))
            } else {
                Box::new(UnknownCommand::new(command.to_string()))
            }
        }
    }
}
