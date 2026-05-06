use std::io::Write;

use crate::command::Token;
use crate::command::{Command, StreamTarget};
use crate::command_type::CommandType;
use crate::shell_state::ShellState;

pub struct Echo {
    pub(super) tokens: Vec<Token>,
}

impl Echo {
    pub(super) fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
}

impl Command for Echo {
    fn execute(&self, _ctx: &mut ShellState) {
        if let StreamTarget::Redirect(redirect) = self.stderr() {
            redirect.touch();
        }
        let output = self.args().join(" ");
        match self.stdout() {
            StreamTarget::Redirect(redirect) => {
                let mut file = redirect.open_write();
                writeln!(file, "{output}").unwrap();
            }
            StreamTarget::Inherit => {
                println!("{output}");
            }
        }
    }

    fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "echo"
    }
}
