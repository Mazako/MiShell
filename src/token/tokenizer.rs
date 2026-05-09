use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

use crate::token::{Input, InputRedirect, RedirectTarget};

#[derive(Parser)]
#[grammar = "token/grammar.pest"]
struct ShellParser;

pub fn parse_input(line: &str) -> Input {
    let parsed = ShellParser::parse(Rule::line, line).unwrap();
    let mut command = String::new();
    let mut args: Vec<String> = Vec::new();
    let mut redirect = None;
    let root = parsed.into_iter().next().unwrap();
    for r in root.into_inner() {
        match r.as_rule() {
            Rule::arg => args.push(arg_to_str(r)),
            Rule::command => command = arg_to_str(r),
            Rule::redirect => redirect = Some(parse_redirect(r)),
            _ => {}
        }
    }

    Input {
        command,
        args,
        redirect,
    }
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

fn arg_to_str(rule: Pair<'_, Rule>) -> String {
    let mut result = String::new();
    for ele in rule.into_inner() {
        if let Rule::segment = ele.as_rule() {
            result.push_str(&segment_to_str(ele));
        }
    }
    result
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
            Rule::escape_char => out.push_str(&sanitize_escape_char_double(ele.as_str())),
            _ => {}
        }
    }
    out
}

fn sanitize_escape_char(input: &str) -> &str {
    &input[1..]
}

fn sanitize_escape_char_double(input: &str) -> String {
    let mut chars = input.chars();
    let _slash = chars.next();
    let Some(next) = chars.next() else {
        return "\\".to_string();
    };
    match next {
        '"' | '\\' | '$' | '`' => next.to_string(),
        '\n' => String::new(),
        _ => {
            let mut out = String::from("\\");
            out.push(next);
            out
        }
    }
}
