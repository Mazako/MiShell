use std::io::Write;

use crate::command::Command;
use crate::command::Token;
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
        let output = self.args().join(" ");
        if let Some(redirect) = self.stdout() {
            let mut file = redirect.open_write();
            writeln!(file, "{output}").unwrap();
        } else {
            println!("{output}");
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
