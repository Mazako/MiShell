use crate::command::{Command, Token};
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
        if args.is_empty() {
            self.print(Some(""), None);
            return;
        }
        let cmd = super::command_from_input(&args[0], ctx);
        let line = match cmd.command_type() {
            CommandType::Builtin => format!("{} is a shell builtin", cmd.name()),
            CommandType::Executable(path) => {
                format!("{} is {}", cmd.name(), path.to_str().unwrap())
            }
            CommandType::Unrecognized => format!("{}: not found", cmd.name()),
        };
        self.print(Some(&line), None);
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
