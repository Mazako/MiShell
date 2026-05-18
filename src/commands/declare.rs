use crate::{command::Command, command_type::CommandType, shell_state::ShellState, token::Input};

pub struct Declare {
    input: Input,
}

impl Declare {
    pub fn new(input: Input) -> Self {
        Declare { input }
    }
}

impl Command for Declare {
    fn input(&self) -> Input {
        self.input.clone()
    }

    fn execute(&self, ctx: &mut ShellState) {
        if let Some(first) = self.input.args.first() {
            if first.as_str() == "-p" {
                if let Some(key) = self.input.args.get(1) {
                    if let Some(value) = ctx.variables.get(key) {
                        self.print(Some(&format!("declare -- {key}=\"{value}\"")), None)
                    } else {
                        self.print(None, Some("declare: variable not found"));
                    }
                }
            } else {
                if let Some((name, value)) = first.split_once('=') {
                    ctx.variables.insert(name.to_string(), value.to_string());
                }
            }
        }
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "declare"
    }
}
