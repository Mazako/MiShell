use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::Stdio;

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

    fn print(&self, stdout: Option<&str>, stderr: Option<&str>) {
        match (stdout, self.stdout()) {
            (Some(stdout), StreamTarget::Redirect(r)) => {
                writeln!(r.open_write(), "{stdout}").unwrap()
            }
            (Some(stdout), StreamTarget::Inherit) => println!("{stdout}"),
            (None, StreamTarget::Redirect(r)) => r.touch(),
            (None, StreamTarget::Inherit) => {}
        }
        match (stderr, self.stderr()) {
            (Some(stderr), StreamTarget::Redirect(r)) => {
                writeln!(r.open_write(), "{stderr}").unwrap()
            }
            (Some(stderr), StreamTarget::Inherit) => eprintln!("{stderr}"),
            (None, StreamTarget::Redirect(r)) => r.touch(),
            (None, StreamTarget::Inherit) => {}
        }
    }

    fn apply_redirects(&self, command: &mut std::process::Command) {
        if let StreamTarget::Redirect(r) = self.stdout() {
            command.stdout(Stdio::from(r.open_write()));
        }
        if let StreamTarget::Redirect(r) = self.stderr() {
            command.stderr(Stdio::from(r.open_write()));
        }
    }

    fn command_type(&self) -> CommandType;

    fn name(&self) -> &str;
}
