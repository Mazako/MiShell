use crate::{command::Command, command_type::CommandType, shell_state::ShellState, token::Input};

pub struct Jobs {
    input: Input,
}

impl Jobs {
    pub fn new(input: Input) -> Self {
        Jobs { input }
    }
}

impl Command for Jobs {
    fn input(&self) -> Input {
        self.input.clone()
    }

    fn execute(&self, ctx: &mut ShellState) {
        ctx.print_background_jobs();
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "jobs"
    }
}
