use crate::command::{Command, Token};
use crate::command_type::CommandType;
use crate::shell_state::ShellState;

pub struct Pwd {
    tokens: Vec<Token>,
}

impl Pwd {
    pub(super) fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
}

impl Command for Pwd {
    fn execute(&self, ctx: &mut ShellState) {
        self.print(Some(ctx.cwd.to_str().unwrap()), None);
    }

    fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "pwd"
    }
}
