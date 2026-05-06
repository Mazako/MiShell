use crate::command::{Command, StreamTarget, Token};
use crate::command_type::CommandType;
use crate::shell_state::ShellState;

pub struct Exit {
    tokens: Vec<Token>,
}

impl Exit {
    pub(super) fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
}

impl Command for Exit {
    fn execute(&self, _ctx: &mut ShellState) {
        if let StreamTarget::Redirect(redirect) = self.stdout() {
            redirect.touch();
        }
        if let StreamTarget::Redirect(redirect) = self.stderr() {
            redirect.touch();
        }
        std::process::exit(0);
    }

    fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "exit"
    }
}
