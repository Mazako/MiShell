use regex::Regex;

use crate::{command_io, shell_state::ShellState, token::Input};

pub fn run(input: &Input, ctx: &mut ShellState) {
    let args = input.args(ctx);
    if let Some(first) = args.first() {
        if first.as_str() == "-p" {
            if let Some(key) = args.get(1) {
                if let Some(value) = ctx.variables.get(key) {
                    command_io::print_stdout(input, &format!("declare -- {key}=\"{value}\""));
                } else {
                    command_io::print_stderr(input, &format!("declare: {key}: not found"));
                }
            }
        } else if let Some((name, value)) = first.split_once('=') {
            if !is_valid_variable_name(name) {
                command_io::print_stderr(
                    input,
                    &format!("declare: `{name}={value}': not a valid identifier"),
                );
            } else {
                ctx.variables.insert(name.to_string(), value.to_string());
            }
        }
    }
}

fn is_valid_variable_name(var: &str) -> bool {
    if let Ok(regex) = Regex::new("^[_a-zA-Z][_0-9a-zA-Z]*$") {
        regex.is_match(var)
    } else {
        false
    }
}
