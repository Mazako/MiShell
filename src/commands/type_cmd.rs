use crate::{
    command_io, command_spec::CommandSpec, command_type::CommandType, shell_state::ShellState,
    token::{Input, parse_line},
};

pub fn run(input: &Input, ctx: &ShellState) {
    let args = input.args(ctx);
    if args.is_empty() {
        command_io::print_stdout(input, "");
        return;
    }
    let line = parse_line(&args[0]);
    let spec = CommandSpec::resolve(line.input, ctx);
    let line = match spec.command_type() {
        CommandType::Builtin => format!("{} is a shell builtin", spec.name()),
        CommandType::Executable(path) => format!("{} is {}", spec.name(), path.to_str().unwrap()),
        CommandType::Unrecognized => format!("{}: not found", spec.name()),
    };
    command_io::print_stdout(input, &line);
}
