mod echo;
mod exec;
mod exit;
mod type_cmd;
mod unknown;
mod pwd;
mod cd;

use std::fs::{self, Metadata};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use crate::command::{Command, Token};
use crate::commands::exec::Exec;
use crate::commands::pwd::Pwd;
use crate::shell_state::ShellState;

pub use cd::Cd;
pub use echo::Echo;
pub use exit::Exit;
pub use type_cmd::Type;
pub use unknown::UnknownCommand;


#[derive(Eq, Debug, PartialEq)]
enum ParseMode {
    Normal,
    Quoted,
    DoubleQuoted
}

fn parse_args(args: &str) -> Vec<Token> {
    let mut tokens: Vec<(String, bool)> = Vec::new();
    let mut current_token = String::new();
    let mut current_token_escaped = false;
    let mut quote_mode = ParseMode::Normal;

    let mut chars = args.trim().chars().peekable();

    while let Some(ch) = chars.next() {
        match quote_mode {
            ParseMode::Normal => {
                if ch == ' ' {
                    push_current_token(&mut tokens, &mut current_token, &mut current_token_escaped);
                } else if ch == '\'' {
                    quote_mode = ParseMode::Quoted;
                    current_token_escaped = true;
                } else if ch == '"' {
                    quote_mode = ParseMode::DoubleQuoted;
                    current_token_escaped = true;
                } else if ch == '\\' {
                    if let Some(next) = chars.next() {
                        current_token_escaped = true;
                        current_token.push(next);
                    }
                } else {
                    current_token.push(ch);
                }
            }
            ParseMode::Quoted => {
                if ch == '\'' {
                    quote_mode = ParseMode::Normal;
                } else {
                    current_token_escaped = true;
                    current_token.push(ch);
                }
            }
            ParseMode::DoubleQuoted => {
                if ch == '"' {
                    quote_mode = ParseMode::Normal;
                } else if ch == '\\' {
                    if let Some(&next) = chars.peek()
                        && matches!(next, '"' | '\\' | '$' | '`' | '\n') {
                            current_token_escaped = true;
                            current_token.push(chars.next().unwrap());
                        } else {
                            current_token.push('\\');
                        }
                } else {
                    current_token_escaped = true;
                    current_token.push(ch);
                }
            }
        }
    }

    push_current_token(&mut tokens, &mut current_token, &mut current_token_escaped);

    tokenize(tokens)
}

fn push_current_token(
    tokens: &mut Vec<(String, bool)>,
    current_token: &mut String,
    current_token_escaped: &mut bool,
) {
    if !current_token.is_empty() {
        tokens.push((std::mem::take(current_token), *current_token_escaped));
        *current_token_escaped = false;
    }
}

fn tokenize(args: Vec<(String, bool)>)  -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = args.into_iter();
    while let Some((arg, escaped)) = iter.next() {
        let token = if escaped {
            Token::Word(arg)
        } else {
            if arg == ">" || arg == "1>" {
                Token::RedirectStdout { path: iter.next().unwrap().0, append: false }
            } else if arg == ">>" || arg == "1>>" {
                Token::RedirectStdout { path: iter.next().unwrap().0, append: true }
            } else if arg == "2>" {
                Token::RedirectStderr { path: iter.next().unwrap().0, append: false }
            } else if arg == "2>>" {
                Token::RedirectStderr { path: iter.next().unwrap().0, append: true }
            } else {
                Token::Word(arg)
            }
        };
        tokens.push(token);
    }
    tokens
}

pub fn command_from_input(input: &str, ctx: &ShellState) -> Box<dyn Command> {
    let parsed_tokens = parse_args(input);
    let mut command = String::new();
    let mut args: Vec<Token> = Vec::new();
    let mut saw_command = false;

    for token in parsed_tokens {
        if !saw_command {
            if let Token::Word(word) = token {
                command = word;
                saw_command = true;
            }
            continue;
        }
        args.push(token);
    }

    println!("{:?}", args);

    match command.as_str() {
        "echo" => Box::new(Echo::new(args)),
        "exit" => Box::new(Exit::new(args)),
        "type" => Box::new(Type::new(args)),
        "pwd" => Box::new(Pwd::new(args)),
        "cd" => Box::new(Cd::new(args)),
        _ => {
            if let Some(path) = find_in_path(&command, &ctx.path_dirs) {
                Box::new(Exec::new(command.to_string(), path, args))
            } else {
                Box::new(UnknownCommand::new(command.to_string(), args))
            }
        }
    }
}

pub fn find_in_path(command: &str, path_dirs: &[PathBuf]) -> Option<PathBuf> {
    for dir in path_dirs {
        if let Ok(res) = fs::read_dir(dir) {
            for e in res.flatten() {
                if let Ok(metadata) = e.metadata()
                    && metadata.is_file()
                    && let Ok(name) = e.file_name().into_string()
                    && name == command
                    && is_executable(metadata)
                {
                    return Some(e.path());
                }
            }
        }
    }
    None
}

fn is_executable(metadata: Metadata) -> bool {
    metadata.permissions().mode() & 0o111 != 0
}
