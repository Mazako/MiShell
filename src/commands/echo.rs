use crate::command::Command;
use crate::command_type::CommandType;
use crate::shell_state::ShellState;
use crate::token::Input;

pub struct Echo {
    pub(super) input: Input,
}

impl Echo {
    pub(super) fn new(input: Input) -> Self {
        Self { input }
    }
}

impl Command for Echo {
    fn execute(&self, ctx: &mut ShellState) {
        self.print_stdout(&self.args(ctx).join(" "));
    }

    fn input(&self) -> Input {
        self.input.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "echo"
    }
}
