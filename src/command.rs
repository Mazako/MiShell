use crate::command_type::CommandType;

pub trait Command {
    fn execute(&self);

    fn args(&self) -> Vec<String>;

    fn command_type(&self) -> CommandType;

    fn name(&self) -> &str;
}
