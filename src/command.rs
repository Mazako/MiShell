use std::fs::{File, OpenOptions};
use std::path::PathBuf;

use crate::command_type::CommandType;
use crate::shell_state::ShellState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StreamTarget {
    Inherit,
    Redirect(Redirect),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Redirect {
    pub path: PathBuf,
    pub append: bool,
}

impl Redirect {
    pub fn open_write(&self) -> File {
        let mut opts = OpenOptions::new();
        opts.create(true).write(true);
        if self.append {
            opts.append(true);
        } else {
            opts.truncate(true);
        }
        opts.open(&self.path).unwrap()
    }

    pub fn touch(&self) {
        let _ = self.open_write();
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Word(String),
    RedirectStdout { path: String, append: bool },
    RedirectStderr { path: String, append: bool },
}

pub trait Command {
    fn execute(&self, ctx: &mut ShellState);

    fn tokens(&self) -> Vec<Token>;

    fn args(&self) -> Vec<String> {
        self.tokens()
            .into_iter()
            .filter_map(|token| match token {
                Token::Word(word) => Some(word),
                _ => None,
            })
            .collect()
    }

    fn stdout(&self) -> StreamTarget {
        self.tokens()
            .into_iter()
            .find_map(|token| match token {
                Token::RedirectStdout { path, append } => Some(StreamTarget::Redirect(Redirect {
                    path: PathBuf::from(path),
                    append,
                })),
                _ => None,
            })
            .unwrap_or(StreamTarget::Inherit)
    }

    fn stderr(&self) -> StreamTarget {
        self.tokens()
            .into_iter()
            .find_map(|token| match token {
                Token::RedirectStderr { path, append } => Some(StreamTarget::Redirect(Redirect {
                    path: PathBuf::from(path),
                    append,
                })),
                _ => None,
            })
            .unwrap_or(StreamTarget::Inherit)
    }

    fn command_type(&self) -> CommandType;

    fn name(&self) -> &str;
}
