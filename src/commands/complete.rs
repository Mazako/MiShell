use std::{collections::HashMap, path::PathBuf};

use thiserror::Error;

use crate::{
    command::{Command, Token},
    command_type::CommandType,
    shell_state::ShellState,
};

#[derive(Debug, Default, Clone)]
pub(super) struct ParsedComplete {
    pub flags: HashMap<String, String>,
    pub target: Option<String>,
}

#[derive(Debug, Error)]
pub(super) enum CompleteParseError {
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

pub(super) struct Complete {
    tokens: Vec<Token>,
}

impl Complete {
    pub(super) fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    fn parse(&self) -> Result<ParsedComplete, CompleteParseError> {
        let words = collect_words(&self.tokens)?;
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
}

fn is_flag_like(word: &str) -> bool {
    word.starts_with('-') && word != "-"
}

fn collect_words(tokens: &[Token]) -> Result<Vec<String>, CompleteParseError> {
    let mut words = Vec::new();
    for token in tokens {
        if let Token::Word(w) = token {
            words.push(w.clone());
        }
    }
    Ok(words)
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

impl Command for Complete {
    fn execute(&self, ctx: &mut ShellState) {
        match self.parse() {
            Ok(parsed) => {
                if let Some(cmd) = parsed.flags.get("-p") {
                    if let Some(path) = ctx.completion_script_for(cmd) {
                        let script_path = path.display().to_string();
                        self.print(Some(&format!("complete -C '{script_path}' {cmd}")), None);
                    } else {
                        self.print(
                            None,
                            Some(&format!("complete: {cmd}: no completion specification")),
                        );
                    }
                    return;
                }
                if let Some(cmd) = parsed.flags.get("-C") {
                    if let Some(target) = &parsed.target {
                        ctx.add_completion_script(target, PathBuf::from(cmd));
                    } else {
                        self.print(None, Some(&format!("complete: {cmd}: no target specified")));
                    }
                }
                if let Some(cmd) = parsed.flags.get("-r") {
                    ctx.remove_completion_script(cmd);
                }
            }
            Err(e) => self.print(None, Some(&format!("{e}"))),
        }
    }

    fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "complete"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Token;

    #[test]
    fn empty_ok() {
        let c = Complete::new(vec![]);
        let p = c.parse().unwrap();
        assert!(p.flags.is_empty());
        assert_eq!(p.target, None);
    }

    #[test]
    fn only_target_ok() {
        let c = Complete::new(vec![Token::Word("git".into())]);
        let p = c.parse().unwrap();
        assert!(p.flags.is_empty());
        assert_eq!(p.target.as_deref(), Some("git"));
    }

    #[test]
    fn pairs_only_no_target_ok() {
        let c = Complete::new(vec![Token::Word("-C".into()), Token::Word("/bin/c".into())]);
        let p = c.parse().unwrap();
        assert_eq!(p.flags.get("-C").map(String::as_str), Some("/bin/c"));
        assert_eq!(p.target, None);
    }

    #[test]
    fn two_flags_and_target_ok() {
        let c = Complete::new(vec![
            Token::Word("-C".into()),
            Token::Word("/bin/c".into()),
            Token::Word("-o".into()),
            Token::Word("default".into()),
            Token::Word("git".into()),
        ]);
        let p = c.parse().unwrap();
        assert_eq!(p.flags.get("-C").map(String::as_str), Some("/bin/c"));
        assert_eq!(p.flags.get("-o").map(String::as_str), Some("default"));
        assert_eq!(p.target.as_deref(), Some("git"));
    }

    #[test]
    fn lone_flag_err() {
        let c = Complete::new(vec![Token::Word("-C".into())]);
        assert!(matches!(
            c.parse(),
            Err(CompleteParseError::FlagWithoutValue(_))
        ));
    }

    #[test]
    fn value_looks_like_flag_err() {
        let c = Complete::new(vec![
            Token::Word("-C".into()),
            Token::Word("-nope".into()),
            Token::Word("git".into()),
        ]);
        assert!(matches!(
            c.parse(),
            Err(CompleteParseError::ValueLooksLikeFlag { .. })
        ));
    }

    #[test]
    fn stray_words_after_pairs_err() {
        let c = Complete::new(vec![
            Token::Word("-C".into()),
            Token::Word("/bin/c".into()),
            Token::Word("a".into()),
            Token::Word("b".into()),
        ]);
        assert!(matches!(c.parse(), Err(CompleteParseError::TrailingWords)));
    }
}
