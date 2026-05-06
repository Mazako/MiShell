use std::{cell::RefCell, rc::Rc};

use rustyline::{
    Helper, Highlighter, Hinter, Validator,
    completion::{Completer, FilenameCompleter, Pair},
};

use crate::{command::Token, commands::parse_args, shell_state::ShellState};

#[derive(Helper, Highlighter, Hinter, Validator)]
pub struct MyHelper {
    pub commands: Vec<String>,
    pub state: Rc<RefCell<ShellState>>,
}

impl MyHelper {
    fn should_complete_files(line: &str, pos: usize, token_count: usize) -> bool {
        token_count > 1 || line.get(pos.saturating_sub(1)..pos) == Some(" ")
    }

    fn complete_files(&self, line: &str, pos: usize) -> rustyline::Result<(usize, Vec<Pair>)> {
        if let Ok((start, pairs)) = FilenameCompleter::new().complete_path(line, pos) {
            let matches = pairs
                .into_iter()
                .map(|pair| {
                    let replacement = if pair.replacement.ends_with('/') {
                        pair.replacement
                    } else {
                        format!("{} ", pair.replacement)
                    };
                    Pair {
                        display: replacement.trim().to_string(),
                        replacement,
                    }
                })
                .collect();
            Ok((start, matches))
        } else {
            Ok((0, Vec::new()))
        }
    }

    fn complete_commands(&self, line: &str, pos: usize, word: &str) -> rustyline::Result<(usize, Vec<Pair>)> {
        let start = line.rmatch_indices(word).next().unwrap().0;
        let prefix = line[start..pos].to_lowercase();
        let state = self.state.borrow();

        let mut candidates: Vec<String> = self
            .commands
            .iter()
            .cloned()
            .chain(state.path_command_names().into_iter().cloned())
            .collect();
        candidates.sort();
        candidates.dedup();

        let matches = candidates
            .into_iter()
            .filter(|candidate| candidate.to_lowercase().starts_with(&prefix))
            .map(|candidate| Pair {
                display: candidate.clone(),
                replacement: format!("{} ", candidate),
            })
            .collect();

        Ok((start, matches))
    }
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
        let Some(last_token) = tokens.last() else {
            return Ok((0, Vec::new()));
        };

        if Self::should_complete_files(line, pos, tokens.len()) {
            return self.complete_files(line, pos);
        }

        match last_token {
            Token::Word(word) => self.complete_commands(line, pos, word),
            _ => Ok((0, Vec::new())),
        }
    }
}
