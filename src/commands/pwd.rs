use std::io::Write;

use crate::{command::{Command, StreamTarget, Token}, command_type::CommandType, shell_state::ShellState};


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
        if let StreamTarget::Redirect(redirect) = self.stderr() {
            redirect.touch();
        }
        let output = ctx.cwd.to_str().unwrap();
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
        "pwd"
    }
}