use crate::command::Command;
use crate::command_type::CommandType;
use crate::shell_state::ShellState;
use crate::token::{Input, parse_line};

pub struct Type {
    pub(super) input: Input,
}

impl Type {
    pub(super) fn new(input: Input) -> Self {
        Self { input }
    }
}

impl Command for Type {
    fn execute(&self, ctx: &mut ShellState) {
        let args = self.args();
        if args.is_empty() {
            self.print(Some(""), None);
            return;
        }
        let line = parse_line(&args[0]);
        let cmd = super::command_from_input(line.input, ctx);
        let line = match cmd.command_type() {
            CommandType::Builtin => format!("{} is a shell builtin", cmd.name()),
            CommandType::Executable(path) => {
                format!("{} is {}", cmd.name(), path.to_str().unwrap())
            }
            CommandType::Unrecognized => format!("{}: not found", cmd.name()),
        };
        self.print(Some(&line), None);
    }

    fn input(&self) -> Input {
        self.input.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "type"
    }
}
