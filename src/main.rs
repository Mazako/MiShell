mod command;
mod command_type;
mod commands;
mod shell_state;

use std::io::{self, Write};

use anyhow::Error;
use commands::command_from_input;
use rustyline::{Config, DefaultEditor, Editor, Helper, Highlighter, Hinter, Validator, completion::{Completer, Pair}};

use crate::{command::Token, commands::parse_args, shell_state::ShellState};

#[derive(Helper, Highlighter, Hinter, Validator)]
pub struct MyHelper { pub commands: Vec<String> }

impl Completer for MyHelper {
    type Candidate = Pair;
    fn complete(&self, line: &str, pos: usize, _: &rustyline::Context<'_>) -> rustyline::Result<(usize, Vec<Pair>)> {
        let tokens = parse_args(line.trim());
        if tokens.last().is_none() {
            return Ok((0, Vec::with_capacity(0)));
        }
        let last = tokens.last().unwrap();
        if let Token::Word(w) = last {
            let start = line.rmatch_indices(w).next().unwrap().0;
            let word = line[start..pos].to_string();
                    let matches: Vec<Pair> = self.commands.iter()
            .filter(|c| c.to_lowercase().starts_with(&word))
            .map(|c| Pair { display: format!("{} ", c.clone()), replacement: format!("{} ", c.clone()) })
            .collect();
            return Ok((start, matches));
        }

        Ok((0, Vec::with_capacity(0)))
    }
}

fn main() -> Result<(), Error>{
    let mut ctx = ShellState::new();
    let cfg = Config::builder().build();
    let mut rl = Editor::with_config(cfg)?;
    let helper = MyHelper{
        commands: vec!["echo".to_string(), "exit".to_string(), "pwd".to_string(), "cd".to_string(), "type".to_string()]
    };
    rl.set_helper(Some(helper));
    loop {
        let input = rl.readline("$ ")?;
        let command = command_from_input(input.trim(), &ctx);
        command.execute(&mut ctx);
    }
}