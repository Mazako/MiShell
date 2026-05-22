use std::{collections::HashMap, path::PathBuf};

use thiserror::Error;

use crate::{command_io, shell_state::ShellState, token::Input};

#[derive(Debug, Default, Clone)]
pub struct ParsedComplete {
    pub flags: HashMap<String, String>,
    pub target: Option<String>,
}

#[derive(Debug, Error)]
pub enum CompleteParseError {
    #[error("complete: flag {0:?} has no value")]
    FlagWithoutValue(String),
    #[error("complete: unexpected extra arguments")]
    TrailingWords,
    #[error("complete: expected a flag starting with '-', got: {0:?}")]
    ExpectedFlag(String),
    #[error("complete: invalid flag name (bare '-' or '--')")]
    InvalidFlag,
    #[error("complete: value for flag {flag:?} looks like another flag (starts with '-')")]
    ValueLooksLikeFlag { flag: String, value: String },
}

pub fn parse(input: &Input, ctx: &ShellState) -> Result<ParsedComplete, CompleteParseError> {
    let words = input.args(ctx);
    let mut flags = HashMap::new();
    let mut target: Option<String> = None;
    let mut iter = words.into_iter().peekable();

    while let Some(word) = iter.next() {
        if is_flag_like(&word) {
            validate_flag(&word)?;
            let Some(value) = iter.next() else {
                return Err(CompleteParseError::FlagWithoutValue(word));
            };
            if is_flag_like(&value) {
                return Err(CompleteParseError::ValueLooksLikeFlag { flag: word, value });
            }
            flags.insert(word, value);
            continue;
        }

        if target.is_some() || iter.peek().is_some() {
            return Err(CompleteParseError::TrailingWords);
        }
        target = Some(word);
    }

    Ok(ParsedComplete { flags, target })
}

fn is_flag_like(word: &str) -> bool {
    word.starts_with('-') && word != "-"
}

fn validate_flag(flag: &str) -> Result<(), CompleteParseError> {
    if !flag.starts_with('-') {
        return Err(CompleteParseError::ExpectedFlag(flag.to_string()));
    }
    if flag == "-" || flag == "--" {
        return Err(CompleteParseError::InvalidFlag);
    }
    Ok(())
}

pub fn run(input: &Input, ctx: &mut ShellState) {
    match parse(input, ctx) {
        Ok(parsed) => {
            if let Some(cmd) = parsed.flags.get("-p") {
                if let Some(path) = ctx.completion_script_for(cmd) {
                    let script_path = path.display().to_string();
                    command_io::print_stdout(
                        input,
                        &format!("complete -C '{script_path}' {cmd}"),
                    );
                } else {
                    command_io::print_stderr(
                        input,
                        &format!("complete: {cmd}: no completion specification"),
                    );
                }
                return;
            }
            if let Some(cmd) = parsed.flags.get("-C") {
                if let Some(target) = &parsed.target {
                    ctx.add_completion_script(target, PathBuf::from(cmd));
                } else {
                    command_io::print_stderr(input, &format!("complete: {cmd}: no target specified"));
                }
            }
            if let Some(cmd) = parsed.flags.get("-r") {
                ctx.remove_completion_script(cmd);
            }
        }
        Err(e) => command_io::print_stderr(input, &format!("{e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Arg, Input};

    fn input(words: &[&str]) -> Input {
        Input::new(
            "complete".to_string(),
            words.iter().map(|w| Arg::String(w.to_string())).collect(),
            None,
        )
    }

    fn ctx() -> ShellState {
        ShellState::new()
    }

    #[test]
    fn empty_ok() {
        let p = parse(&input(&[]), &ctx()).unwrap();
        assert!(p.flags.is_empty());
        assert_eq!(p.target, None);
    }

    #[test]
    fn only_target_ok() {
        let p = parse(&input(&["git"]), &ctx()).unwrap();
        assert!(p.flags.is_empty());
        assert_eq!(p.target.as_deref(), Some("git"));
    }

    #[test]
    fn pairs_only_no_target_ok() {
        let p = parse(&input(&["-C", "/bin/c"]), &ctx()).unwrap();
        assert_eq!(p.flags.get("-C").map(String::as_str), Some("/bin/c"));
        assert_eq!(p.target, None);
    }

    #[test]
    fn two_flags_and_target_ok() {
        let p = parse(&input(&["-C", "/bin/c", "-o", "default", "git"]), &ctx()).unwrap();
        assert_eq!(p.flags.get("-C").map(String::as_str), Some("/bin/c"));
        assert_eq!(p.flags.get("-o").map(String::as_str), Some("default"));
        assert_eq!(p.target.as_deref(), Some("git"));
    }

    #[test]
    fn lone_flag_err() {
        assert!(matches!(
            parse(&input(&["-C"]), &ctx()),
            Err(CompleteParseError::FlagWithoutValue(_))
        ));
    }

    #[test]
    fn value_looks_like_flag_err() {
        assert!(matches!(
            parse(&input(&["-C", "-nope", "git"]), &ctx()),
            Err(CompleteParseError::ValueLooksLikeFlag { .. })
        ));
    }

    #[test]
    fn stray_words_after_pairs_err() {
        assert!(matches!(
            parse(&input(&["-C", "/bin/c", "a", "b"]), &ctx()),
            Err(CompleteParseError::TrailingWords)
        ));
    }
}
