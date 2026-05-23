use crate::{command_io, shell_state::ShellState, token::Input};

pub fn run(input: &Input, ctx: &ShellState) {
    let mut args = input.args(ctx);
    let interpret = args.first().is_some_and(|a| a == "-e");
    if interpret {
        args.remove(0);
    }

    let text = args.join(" ");
    let output = if interpret {
        interpret_escapes(&text)
    } else {
        text
    };

    command_io::print_stdout(input, &output);
}

fn interpret_escapes(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some('\\') => out.push('\\'),
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    out
}
