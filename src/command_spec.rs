use std::path::PathBuf;
use std::process::Child;

use crate::command_type::CommandType;
use crate::commands::{
    cd, complete, declare, echo, exec, exit, history, jobs, pwd, type_cmd, unknown,
};
use crate::shell_state::ShellState;
use crate::token::Input;

/// Resolved command: parsed line data (`Input`) + which handler runs.
#[derive(Debug, Clone)]
pub enum CommandSpec {
    Echo(Input),
    Exit(Input),
    Type(Input),
    Pwd(Input),
    Cd(Input),
    Complete(Input),
    Jobs(Input),
    History(Input),
    Declare(Input),
    Exec { input: Input, path: PathBuf },
    Unknown(Input),
}

impl CommandSpec {
    pub fn resolve(input: Input, ctx: &ShellState) -> Self {
        match input.command.as_str() {
            "echo" => Self::Echo(input),
            "exit" => Self::Exit(input),
            "type" => Self::Type(input),
            "pwd" => Self::Pwd(input),
            "cd" => Self::Cd(input),
            "complete" => Self::Complete(input),
            "jobs" => Self::Jobs(input),
            "history" => Self::History(input),
            "declare" => Self::Declare(input),
            name => {
                if let Some(path) = ctx.find_in_path(name) {
                    Self::Exec { input, path }
                } else {
                    Self::Unknown(input)
                }
            }
        }
    }

    pub fn input(&self) -> &Input {
        match self {
            Self::Echo(i)
            | Self::Exit(i)
            | Self::Type(i)
            | Self::Pwd(i)
            | Self::Cd(i)
            | Self::Complete(i)
            | Self::Jobs(i)
            | Self::History(i)
            | Self::Declare(i)
            | Self::Unknown(i) => i,
            Self::Exec { input, .. } => input,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Echo(_) => "echo",
            Self::Exit(_) => "exit",
            Self::Type(_) => "type",
            Self::Pwd(_) => "pwd",
            Self::Cd(_) => "cd",
            Self::Complete(_) => "complete",
            Self::Jobs(_) => "jobs",
            Self::History(_) => "history",
            Self::Declare(_) => "declare",
            Self::Exec { input, .. } | Self::Unknown(input) => input.command.as_str(),
        }
    }

    pub fn command_type(&self) -> CommandType {
        match self {
            Self::Echo(_)
            | Self::Exit(_)
            | Self::Type(_)
            | Self::Pwd(_)
            | Self::Cd(_)
            | Self::Complete(_)
            | Self::Jobs(_)
            | Self::History(_)
            | Self::Declare(_) => CommandType::Builtin,
            Self::Exec { path, .. } => CommandType::Executable(path.clone()),
            Self::Unknown(_) => CommandType::Unrecognized,
        }
    }

    pub fn args(&self, ctx: &ShellState) -> Vec<String> {
        self.input().args(ctx)
    }

    pub fn execute(&self, ctx: &mut ShellState) {
        match self {
            Self::Echo(input) => echo::run(input, ctx),
            Self::Exit(input) => exit::run(input, ctx),
            Self::Type(input) => type_cmd::run(input, ctx),
            Self::Pwd(input) => pwd::run(input, ctx),
            Self::Cd(input) => cd::run(input, ctx),
            Self::Complete(input) => complete::run(input, ctx),
            Self::Jobs(input) => jobs::run(input, ctx),
            Self::History(input) => history::run(input, ctx),
            Self::Declare(input) => declare::run(input, ctx),
            Self::Exec { input, path } => exec::run(input, path, ctx),
            Self::Unknown(input) => unknown::run(input, ctx),
        }
    }

    pub fn execute_background(&self, ctx: &mut ShellState) -> Child {
        match self {
            Self::Exec { input, .. } => exec::spawn_background(input, ctx),
            _ => {
                let curr_exe = std::env::current_exe().unwrap();
                std::process::Command::new(curr_exe)
                    .args(self.args(ctx))
                    .spawn()
                    .unwrap()
            }
        }
    }
}
