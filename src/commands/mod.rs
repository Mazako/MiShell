mod cd;
mod echo;
mod exec;
mod exit;
mod pwd;
mod tokenizer;
mod type_cmd;
mod unknown;
mod complete;

use crate::command::{Command, Token};
use crate::commands::exec::Exec;
use crate::commands::pwd::Pwd;
use crate::shell_state::ShellState;

pub use cd::Cd;
pub use echo::Echo;
pub use exit::Exit;
pub use tokenizer::parse_args;
pub use type_cmd::Type;
pub use unknown::UnknownCommand;

use crate::commands::complete::Complete;

pub fn command_from_input(input: &str, ctx: &ShellState) -> Box<dyn Command> {
    let parsed_tokens = parse_args(input);
    let mut command = String::new();
    let mut args: Vec<Token> = Vec::new();
    let mut saw_command: bool = false;

    for token in parsed_tokens {
        if !saw_command {
            if let Token::Word(word) = token {
                command = word;
                saw_command = true;
            }
            continue;
        }
        args.push(token);
    }

    match command.as_str() {
        "echo" => Box::new(Echo::new(args)),
        "exit" => Box::new(Exit::new(args)),
        "type" => Box::new(Type::new(args)),
        "pwd" => Box::new(Pwd::new(args)),
        "cd" => Box::new(Cd::new(args)),
        "complete" => Box::new(Complete::new(args)),
        _ => {
            if let Some(path) = ctx.find_in_path(&command) {
                Box::new(Exec::new(command.to_string(), path, args))
            } else {
                Box::new(UnknownCommand::new(command.to_string(), args))
            }
        }
    }
}
