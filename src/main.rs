mod command;
mod command_type;
mod commands;
mod my_helper;
mod shell_state;
mod token;

use std::{alloc::System, cell::RefCell, env::args, os::unix::process, process::exit, rc::Rc};

use anyhow::Error;
use commands::command_from_input;
use my_helper::MyHelper;
use rustyline::{CompletionType, Config, Editor, config::BellStyle};

use crate::shell_state::ShellState;

fn main() -> Result<(), Error> {
    let state = Rc::new(RefCell::new(ShellState::new()));

    let args: Vec<String> = args().collect();

    if args.len() > 1 {
        let input = args[1..].join(" ");
        let mut s = state.borrow_mut();
        let command = command_from_input(input.trim(), &s);
        command.execute(&mut s);
        exit(0)
    }

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
        if command.input().background  {
            let child = command.execute_background();
            let (id, pid) = state.add_child(child, &input);
            println!("[{id}] {pid}")

        } else {
            command.execute(&mut state);
        }
    }
}
