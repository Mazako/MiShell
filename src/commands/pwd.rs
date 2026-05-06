use std::io::Write;

use crate::{command::{Command, Token}, command_type::CommandType, shell_state::ShellState};


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
        let output = ctx.cwd.to_str().unwrap();
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
        "pwd"
    }
}