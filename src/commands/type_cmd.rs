use std::io::Write;

use crate::command::Command;
use crate::command::Token;
use crate::command_type::CommandType;
use crate::shell_state::ShellState;

pub struct Type {
    pub(super) tokens: Vec<Token>,
}

impl Type {
    pub(super) fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
}

impl Command for Type {
    fn execute(&self, ctx: &mut ShellState) {
        let args = self.args();
        let write_out = |line: String, redirect: Option<crate::command::Redirect>| {
            if let Some(redirect) = redirect {
                let mut file = redirect.open_write();
                writeln!(file, "{line}").unwrap();
            } else {
                println!("{line}");
            }
        };

        if args.is_empty() {
            write_out(String::new(), self.stdout());
            return;
        }
        let cmd = super::command_from_input(&args[0], ctx);
        match cmd.command_type() {
            CommandType::Builtin => write_out(format!("{} is a shell builtin", cmd.name()), self.stdout()),
            CommandType::Executable(path) => write_out(format!("{} is {}", cmd.name(), path.to_str().unwrap()), self.stdout()),
            CommandType::Unrecognized => write_out(format!("{}: not found", cmd.name()), self.stdout()),
        }
    }

    fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "type"
    }
}
