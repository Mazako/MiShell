use std::{cell::RefCell, collections::HashMap, path::PathBuf, process::Command, rc::Rc};

use rustyline::{
    Helper, Highlighter, Hinter, Validator,
    completion::{Completer, FilenameCompleter, Pair},
};

use crate::{
    shell_state::ShellState,
    token::{Input, parse_line},
};

#[derive(Helper, Highlighter, Hinter, Validator)]
pub struct MyHelper {
    pub commands: Vec<String>,
    pub state: Rc<RefCell<ShellState>>,
}

impl MyHelper {
    fn complete_inner(&self, line: &str, pos: usize) -> rustyline::Result<(usize, Vec<Pair>)> {
        if line.trim().is_empty() {
            return Ok((0, Vec::new()));
        }

        //TODO add suppport for completion
        let tokens = parse_line(line).input;

        let (target, prev1, prev2) = triplet(line, pos, &tokens);

        let Some(target_str) = target else {
            return Ok((0, Vec::new()));
        };

        let Some(prev1_str) = prev1 else {
            return self.complete_commands(
                line,
                pos,
                target_str,
                self.regular_complete_candidates(),
                false,
            );
        };

        let state = self.state.borrow();
        if let Some(completion_path) = state.completion_script_for(prev1_str) {
            return self.complete_commands(
                line,
                pos,
                target_str,
                self.custom_complete_candidates(
                    completion_path,
                    prev1_str,
                    target_str,
                    prev1_str,
                    line,
                    pos,
                ),
                true,
            );
        }

        if let Some(prev2_str) = prev2
            && let Some(completion_path) = state.completion_script_for(prev2_str)
        {
            return self.complete_commands(
                line,
                pos,
                target_str,
                self.custom_complete_candidates(
                    completion_path,
                    prev2_str,
                    target_str,
                    prev1_str,
                    line,
                    pos,
                ),
                true,
            );
        }
        self.complete_files(line, pos)
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

    fn custom_complete_candidates(
        &self,
        path: &PathBuf,
        command: &str,
        word: &str,
        prev_word: &str,
        comp_line: &str,
        comp_point: usize,
    ) -> Vec<String> {
        let comp_point_str = comp_point.to_string();
        let envs: HashMap<&str, &str> = [("COMP_LINE", comp_line), ("COMP_POINT", &comp_point_str)]
            .into_iter()
            .collect();

        let program: Result<std::process::Output, std::io::Error> = Command::new(path)
            .args(vec![command, word, prev_word])
            .envs(envs)
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
            if allow_empty && cand.is_empty() {
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
        self.complete_inner(line, pos)
    }
}

fn triplet<'a>(
    line: &'a str,
    pos: usize,
    input: &'a Input,
) -> (Option<&'a str>, Option<&'a str>, Option<&'a str>) {
    let (first, second, third) = input.last_three_elements();
    if line.get(pos.saturating_sub(1)..pos) == Some(" ") {
        return (Some(""), Some(first), second);
    }

    (Some(first), second, third)
}

#[cfg(test)]
#[path = "tests/my_helper_tests.rs"]
mod tests;
