mod command;
mod command_type;
mod commands;
mod my_helper;
mod shell_state;

use std::{cell::RefCell, rc::Rc};

use anyhow::Error;
use commands::command_from_input;
use my_helper::MyHelper;
use rustyline::{CompletionType, Config, Editor, config::BellStyle};

use crate::shell_state::ShellState;

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
