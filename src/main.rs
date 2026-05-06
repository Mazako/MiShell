mod command;
mod command_type;
mod commands;
mod shell_state;

use std::{cell::RefCell, rc::Rc};

use anyhow::Error;
use commands::command_from_input;
use rustyline::{
    CompletionType, Config, Editor, Helper, Highlighter, Hinter, Validator,
    completion::{Completer, FilenameCompleter, Pair},
    config::BellStyle,
};

use crate::{command::Token, commands::parse_args, shell_state::ShellState};

#[derive(Helper, Highlighter, Hinter, Validator)]
pub struct MyHelper {
    pub commands: Vec<String>,
    pub state: Rc<RefCell<ShellState>>,
}

impl Completer for MyHelper {
    type Candidate = Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        _: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let tokens = parse_args(line.trim());
        if tokens.last().is_none() {
            return Ok((0, Vec::with_capacity(0)));
        }
        let last = tokens.last().unwrap();
        if tokens.len() > 1 || &line[pos - 1..pos] == " " {
            if let Ok((size, pairs)) = FilenameCompleter::new().complete_path(line, pos) {
                let mapped: Vec<Pair> = pairs
                    .into_iter()
                    .map(|f| {
                        let replacement = if f.replacement.ends_with("/") {
                            f.replacement
                        } else {
                            format!("{} ", f.replacement)
                        };
                        Pair {
                            display: f.display,
                            replacement,
                        }
                    })
                    .collect();
                return Ok((size, mapped));
            } else {
                return Ok((0, Vec::with_capacity(0)));
            }
        }
        if let Token::Word(w) = last {
            let start = line.rmatch_indices(w).next().unwrap().0;
            let word = line[start..pos].to_string();
            let state = self.state.borrow();
            let mut possible_hints =
                [self.commands.iter().collect(), state.path_command_names()].concat();
            possible_hints.sort();
            let matches: Vec<Pair> = possible_hints
                .into_iter()
                .filter(|c: &&String| c.to_lowercase().starts_with(&word))
                .map(|c| Pair {
                    display: c.clone().to_string(),
                    replacement: format!("{} ", c.clone()),
                })
                .collect();
            return Ok((start, matches));
        }

        Ok((0, Vec::with_capacity(0)))
    }
}

fn main() -> Result<(), Error> {
    let state = Rc::new(RefCell::new(ShellState::new()));
    let cfg = Config::builder()
        .completion_type(CompletionType::List)
        .bell_style(BellStyle::Audible)
        .build();
    let mut rl = Editor::with_config(cfg)?;
    let helper = MyHelper {
        commands: vec![
            "echo".to_string(),
            "exit".to_string(),
            "pwd".to_string(),
            "cd".to_string(),
            "type".to_string(),
        ],
        state: Rc::clone(&state),
    };
    rl.set_helper(Some(helper));
    loop {
        let input = rl.readline("$ ")?;
        let mut state = state.borrow_mut();
        let command = command_from_input(input.trim(), &state);
        command.execute(&mut state);
    }
}
