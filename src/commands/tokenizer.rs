use crate::command::Token;

#[derive(Eq, Debug, PartialEq)]
enum ParseMode {
    Normal,
    Quoted,
    DoubleQuoted,
}

pub fn parse_args(args: &str) -> Vec<Token> {
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
                        && matches!(next, '"' | '\\' | '$' | '`' | '\n')
                    {
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

fn tokenize(args: Vec<(String, bool)>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = args.into_iter();
    while let Some((arg, escaped)) = iter.next() {
        let token = if escaped {
            Token::Word(arg)
        } else if arg == ">" || arg == "1>" {
            Token::RedirectStdout {
                path: iter.next().unwrap().0,
                append: false,
            }
        } else if arg == ">>" || arg == "1>>" {
            Token::RedirectStdout {
                path: iter.next().unwrap().0,
                append: true,
            }
        } else if arg == "2>" {
            Token::RedirectStderr {
                path: iter.next().unwrap().0,
                append: false,
            }
        } else if arg == "2>>" {
            Token::RedirectStderr {
                path: iter.next().unwrap().0,
                append: true,
            }
        } else {
            Token::Word(arg)
        };
        tokens.push(token);
    }
    tokens
}
