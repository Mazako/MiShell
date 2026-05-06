mod command;
mod command_type;
mod commands;
mod shell_state;

use std::{cell::RefCell, rc::Rc};

use anyhow::Error;
use commands::command_from_input;
use rustyline::{
    Config, Editor, Helper, Highlighter, Hinter, Validator,
    completion::{Completer, Pair},
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
        if let Token::Word(w) = last {
            let start = line.rmatch_indices(w).next().unwrap().0;
            let word = line[start..pos].to_string();
            let state = self.state.borrow();
            let matches: Vec<Pair> = self
                .commands
                .iter()
                .chain(state.path_command_names())
                .filter(|c: &&String| c.to_lowercase().starts_with(&word))
                .map(|c| Pair {
                    display: format!("{} ", c.clone()),
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
    let cfg = Config::builder().build();
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
