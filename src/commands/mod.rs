mod cd;
mod complete;
mod echo;
mod exec;
mod exit;
mod jobs;
mod pwd;
mod type_cmd;
mod unknown;
mod history;
mod declare;

use crate::command::Command;
use crate::commands::declare::Declare;
use crate::commands::exec::Exec;
use crate::commands::history::History;
use crate::commands::jobs::Jobs;
use crate::commands::pwd::Pwd;
use crate::shell_state::ShellState;
use crate::token::Input;

pub use cd::Cd;
pub use echo::Echo;
pub use exit::Exit;
pub use type_cmd::Type;
pub use unknown::UnknownCommand;

use crate::commands::complete::Complete;

pub fn command_from_input(input: Input, ctx: &ShellState) -> Box<dyn Command> {
    let command = input.command.as_str();
    match command {
        "echo" => Box::new(Echo::new(input)),
        "exit" => Box::new(Exit::new(input)),
        "type" => Box::new(Type::new(input)),
        "pwd" => Box::new(Pwd::new(input)),
        "cd" => Box::new(Cd::new(input)),
        "complete" => Box::new(Complete::new(input)),
        "jobs" => Box::new(Jobs::new(input)),
        "history" => Box::new(History::new(input)),
        "declare" => Box::new(Declare::new(input)),
        _ => {
            if let Some(path) = ctx.find_in_path(command) {
                Box::new(Exec::new(input, path))
            } else {
                Box::new(UnknownCommand::new(input))
            }
        }
    }
}
