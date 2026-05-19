use crate::command::Command;
use crate::command_type::CommandType;
use crate::shell_state::ShellState;
use crate::token::Input;

pub struct Pwd {
    input: Input,
}

impl Pwd {
    pub(super) fn new(input: Input) -> Self {
        Self { input }
    }
}

impl Command for Pwd {
    fn execute(&self, ctx: &mut ShellState) {
        self.print_stdout(ctx.cwd.to_str().unwrap());
    }

    fn input(&self) -> Input {
        self.input.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "pwd"
    }
}
