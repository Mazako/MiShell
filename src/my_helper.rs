use std::{cell::RefCell, path::PathBuf, process::Command, rc::Rc};

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

    fn custom_complete_candidates(
        &self,
        path: &PathBuf,
        command: &str,
        word: &str,
        prev_word: &str,
    ) -> Vec<String> {
        let program = Command::new(path)
            .args(vec![command, word, prev_word])
            .output();
        if let Ok(output) = program {
            return String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(|f| f.trim().to_string())
                .collect();
        }
        Vec::new()
    }

    fn regular_complete_candidates(&self) -> Vec<String> {
        let state = self.state.borrow();
        let mut completions: Vec<String> = self
            .commands
            .iter()
            .cloned()
            .chain(state.path_command_names().into_iter().cloned())
            .collect();
        completions.sort();
        completions.dedup();
        completions
    }

    fn complete_commands(
        &self,
        line: &str,
        pos: usize,
        word: &str,
        candidates: Vec<String>,
        allow_empty: bool,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let start = line.rmatch_indices(word).next().unwrap().0;
        let prefix = line[start..pos].to_lowercase();
        let fil = |cand: &String| {
            if allow_empty {
                true
            } else {
                cand.to_lowercase().starts_with(&prefix)
            }
        };
        let matches = candidates
            .into_iter()
            .filter(fil)
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
        if line.trim().is_empty() {
            return Ok((0, Vec::new()));
        }

        let tokens = parse_args(line.trim());

        let (prev2, prev1, target) = triplet(line, pos, &tokens);

        let Some(target_str) = target else {
            return Ok((0, Vec::new()));
        };

        let Some(prev1_str) = prev1 else {
            return self.complete_commands(
                line,
                pos,
                &target_str,
                self.regular_complete_candidates(),
                false,
            );
        };

        let state = self.state.borrow();
        if let Some(completion_path) = state.completion_script_for(&prev1_str) {
            return self.complete_commands(
                line,
                pos,
                &target_str,
                self.custom_complete_candidates(completion_path, &prev1_str, &target_str, ""),
                true,
            );
        }

        if let Some(prev2_str) = prev2
            && let Some(completion_path) = state.completion_script_for(&prev2_str)
        {
            return self.complete_commands(
                line,
                pos,
                &target_str,
                self.custom_complete_candidates(
                    completion_path,
                    &prev2_str,
                    &target_str,
                    &prev1_str,
                ),
                true,
            );
        }
        self.complete_files(line, pos)
    }
}

fn triplet(
    line: &str,
    pos: usize,
    tokens: &[Token],
) -> (Option<String>, Option<String>, Option<String>) {
    let mut words: Vec<String> = tokens.iter().flat_map(|t| t.to_simple_string()).collect();
    if line.get(pos.saturating_sub(1)..pos) == Some(" ") {
        words.push(String::new());
    }
    let first = words.pop();
    let second = words.pop();
    let third = words.pop();

    (third, second, first)
}
