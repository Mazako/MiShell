use std::collections::HashMap;

use thiserror::Error;

use crate::{
    command::{Command, Token},
    command_type::CommandType,
    shell_state::ShellState,
};

#[derive(Debug, Default, Clone)]
pub(super) struct ParsedComplete {
    pub flags: HashMap<String, String>,
    pub target: String,
}

#[derive(Debug, Error)]
pub(super) enum CompleteParseError {
    #[error("complete: expected `-flag value` pairs followed by one command name (or a lone command name)")]
    MissingTargetOrValue,
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

        if words.is_empty() {
            return Ok(ParsedComplete::default());
        }

        if words.len() % 2 == 0 {
            return Err(CompleteParseError::MissingTargetOrValue);
        }

        let n = words.len();
        let target = words[n - 1].clone();
        let mut flags = HashMap::new();

        let mut i = 0;
        while i + 2 < n {
            let flag = &words[i];
            let value = &words[i + 1];

            validate_flag(flag)?;
            if value.starts_with('-') && value != "-" {
                return Err(CompleteParseError::ValueLooksLikeFlag {
                    flag: flag.clone(),
                    value: value.clone(),
                });
            }

            flags.insert(flag.clone(), value.clone());
            i += 2;
        }

        Ok(ParsedComplete {
            flags,
            target: target.clone(),
        })
    }
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
    fn execute(&self, _ctx: &mut ShellState) {
        match self.parse() {
            Ok(_) => {}
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
        assert!(p.target.is_empty());
    }

    #[test]
    fn only_target_ok() {
        let c = Complete::new(vec![Token::Word("git".into())]);
        let p = c.parse().unwrap();
        assert!(p.flags.is_empty());
        assert_eq!(p.target, "git");
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
        assert_eq!(p.target, "git");
    }

    #[test]
    fn even_word_count_err() {
        let c = Complete::new(vec![
            Token::Word("-C".into()),
            Token::Word("/bin/c".into()),
        ]);
        assert!(matches!(
            c.parse(),
            Err(CompleteParseError::MissingTargetOrValue)
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
}
