use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

use crate::token::{Input, InputRedirect, Line, RedirectTarget, arg_type::Arg};

#[derive(Parser)]
#[grammar = "token/grammar.pest"]
struct ShellParser;

fn print_ast(pair: Pair<Rule>, indent: usize) {
    let spacing = "  ".repeat(indent);

    let rule = pair.as_rule();
    let str_val = pair.as_str();

    let mut inner = pair.into_inner();
    let first_child = inner.next();

    match first_child {
        Some(first) => {
            println!("{}{:?}", spacing, rule);
            print_ast(first, indent + 1);
            for child in inner {
                print_ast(child, indent + 1);
            }
        }
        None => {
            println!("{}{:?}: {:?}", spacing, rule, str_val);
        }
    }
}

pub fn parse_line(line: &str) -> Line {
    let parsed = ShellParser::parse(Rule::line, line).unwrap();
    let root = parsed.into_iter().next().unwrap();
    // print_ast(root.clone(), 0);
    let mut background = false;
    let mut input = None;
    let mut pipes: Vec<Input> = Vec::new();
    for r in root.into_inner() {
        match r.as_rule() {
            Rule::input => input = Some(parse_input(r)),
            Rule::pipe => pipes.push(parse_input(r.into_inner().next().unwrap())),
            Rule::background => background = true,
            _ => {}
        }
    }
    let input = input.unwrap();
    Line {
        input,
        pipes,
        background,
    }
}

pub fn parse_input(rule: Pair<'_, Rule>) -> Input {
    let mut command = String::new();
    let mut args: Vec<Arg> = Vec::new();
    let mut redirect = None;

    for r in rule.into_inner() {
        match r.as_rule() {
            Rule::arg => args.push(arg_to_str(r)),
            Rule::command => command = segment_to_str(r.into_inner().next().unwrap()),
            Rule::redirect => redirect = Some(parse_redirect(r)),
            _ => {}
        }
    }

    Input::new(command, args, redirect)
}

fn parse_redirect(rule: Pair<'_, Rule>) -> InputRedirect {
    let mode = rule.clone().into_inner().next().unwrap();
    let path = segment_to_str(
        rule.into_inner()
            .next()
            .unwrap()
            .into_inner()
            .next()
            .unwrap(),
    )
    .to_string();
    match mode.as_rule() {
        Rule::write_stdout => InputRedirect {
            target: RedirectTarget::Stdout,
            path,
            append: false,
        },
        Rule::write_stderr => InputRedirect {
            target: RedirectTarget::Stderr,
            path,
            append: false,
        },
        Rule::override_stdout => InputRedirect {
            target: RedirectTarget::Stdout,
            path,
            append: true,
        },
        Rule::override_sterr => InputRedirect {
            target: RedirectTarget::Stderr,
            path,
            append: true,
        },
        _ => panic!(),
    }
}

fn arg_to_str(rule: Pair<'_, Rule>) -> Arg {
    let parts: Vec<Arg> = rule.into_inner().map(segment_to_arg).collect();
    match parts.len() {
        0 => panic!("empty arg"),
        1 => parts.into_iter().next().unwrap(),
        _ => Arg::concat(parts),
    }
}

fn segment_to_arg(segment: Pair<'_, Rule>) -> Arg {
    let inner = segment.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::word => Arg::String(word_to_str(inner)),
        Rule::quoted_part => Arg::String(sanitize_quoted_part(inner.as_str()).to_string()),
        Rule::double_quoted_part => Arg::String(double_quoted_to_str(inner)),
        Rule::variable => Arg::Variable(inner.as_str()[1..].to_string()),
        Rule::braces_variable => Arg::Variable(braces_variable_to_str(inner.as_str())),
        _ => panic!("unexpected segment content"),
    }
}

fn braces_variable_to_str(variable: &str) -> String {
    if variable == "${}" {
        String::new()
    } else {
        variable[2..variable.len()-1].to_string()
    }
}


fn segment_to_str(rule: Pair<'_, Rule>) -> String {
    let next = rule.into_inner().next().unwrap();
    match next.as_rule() {
        Rule::word => word_to_str(next),
        Rule::quoted_part => sanitize_quoted_part(next.as_str()).to_string(),
        Rule::double_quoted_part => double_quoted_to_str(next),
        _ => panic!(),
    }
}


fn word_to_str(rule: Pair<'_, Rule>) -> String {
    let mut result = String::new();
    for ele in rule.into_inner() {
        if let Rule::regular_part = ele.as_rule() {
            result.push_str(ele.as_str());
        } else if let Rule::escape_char = ele.as_rule() {
            result.push_str(sanitize_escape_char(ele.as_str()));
        }
    }
    result
}

fn sanitize_quoted_part(input: &str) -> &str {
    &input[1..input.len() - 1]
}

fn double_quoted_to_str(rule: Pair<'_, Rule>) -> String {
    let mut out = String::new();
    for ele in rule.into_inner() {
        match ele.as_rule() {
            Rule::double_quoted_regular => out.push_str(ele.as_str()),
            Rule::double_quoted_escape_interpreted if ele.as_str() != "\\\n" => {
                out.push_str(sanitize_escape_char(ele.as_str()))
            }
            Rule::double_quoted_escape_literal => out.push_str(ele.as_str()),
            _ => {}
        }
    }
    out
}

fn sanitize_escape_char(input: &str) -> &str {
    &input[1..]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell_state::ShellState;
    use crate::token::arg_type::Arg;

    #[test]
    fn parse_line_with_trailing_cr() {
        parse_line("custom_exe_1806 $Raspberry_4 apple_$Blueberry_6\r");
    }

    #[test]
    fn parse_from_joined_argv() {
        let parts = vec!["custom_exe_1806 $Raspberry_4 apple_$Blueberry_6".to_string()];
        parse_line(&parts.join(" "));
    }

    #[test]
    fn parse_braces_variable() {
        let line = parse_line("echo ${a}");
        assert_eq!(line.input.args(&ShellState::new()).len(), 1);
    }

    #[test]
    fn parse_embedded_variable_in_arg() {
        let line = parse_line("custom_exe_1806 $Raspberry_4 apple_$Blueberry_6");
        let mut state = ShellState::new();
        state.variables.insert("Raspberry_4".into(), "banana".into());
        state.variables.insert("Blueberry_6".into(), "strawberry".into());
        let args = line.input.args(&state);
        assert_eq!(args, vec!["banana", "apple_strawberry"]);
    }
}
