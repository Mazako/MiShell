mod command_io;
mod command_spec;
mod command_type;
mod commands;
mod my_helper;
mod shell_history;
mod shell_state;
mod token;

use std::{
    cell::RefCell,
    env::{self, args},
    io::Read,
    path::{Path, PathBuf},
    process::{Stdio, exit},
    rc::Rc,
};

use anyhow::Error;
use command_spec::CommandSpec;
use command_type::CommandType::{Builtin, Executable, Unrecognized};
use my_helper::MyHelper;
use rustyline::{CompletionType, Config, Editor, config::BellStyle};
use shell_history::{SharedShellHistory, ShellHistory};
use shell_state::ShellState;
use token::{Line, parse_line};

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();

    if args.len() > 1 {
        let mut state = ShellState::new();
        let input = args[1..].join(" ");
        let command = CommandSpec::resolve(parse_line(&input).input, &state);
        command.execute(&mut state);
        exit(0)
    }

    let history_rc = Rc::new(RefCell::new(ShellHistory::default()));
    let state = Rc::new(RefCell::new(ShellState::with_history(Rc::clone(
        &history_rc,
    ))));

    let cfg = Config::builder()
        .completion_type(CompletionType::List)
        .bell_style(BellStyle::Audible)
        .auto_add_history(true)
        .build();
    let mut rl = Editor::with_history(cfg, SharedShellHistory(Rc::clone(&history_rc)))?;
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
    with_histfile(&mut rl, |r, p| {
        let _ = r.load_history(p);
    });
    loop {
        let mut state_mut = state.borrow_mut();
        state_mut.print_and_reap(true);
        drop(state_mut);
        let input = rl.readline("$ ")?;
        if input.trim().is_empty() {
            continue;
        }
        let mut state_mut = state.borrow_mut();
        let line = parse_line(&input);
        let command = CommandSpec::resolve(line.input, &state_mut);

        if line.background {
            let child = command.execute_background(&mut state_mut);
            let (id, pid) = state_mut.add_child(child, &input);
            println!("[{id}] {pid}")
        } else {
            command.execute(&mut state_mut);
        }
        if !state_mut.running {
            break;
        }
    }
    with_histfile(&mut rl, |r, p| {
        let _ = r.save_history(p);
    });
    Ok(())
}

fn with_histfile(
    rl: &mut Editor<MyHelper, SharedShellHistory>,
    mut fun: impl FnMut(&mut Editor<MyHelper, SharedShellHistory>, &Path),
) {
    if let Ok(file) = std::env::var("HISTFILE") {
        fun(rl, Path::new(&file));
    }
}

//TODO: Implement when more time :)
fn pipeline(line: Line, state_mut: &mut ShellState) -> std::io::Result<()> {
    let commands: Vec<CommandSpec> = std::iter::once(line.input)
        .chain(line.pipes)
        .map(|input| CommandSpec::resolve(input, state_mut))
        .collect();

    let first_command = &commands[0];
    let (path, args) = exec_and_args(first_command, state_mut);
    let mut child = std::process::Command::new(path)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()?;

    for ele in &commands[1..] {
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| std::io::Error::other("Cannot open stdout"))?;
        let (path, args) = exec_and_args(ele, state_mut);
        child = std::process::Command::new(path)
            .args(args)
            .stdout(Stdio::piped())
            .stdin(Stdio::from(stdout))
            .spawn()?;
    }
    let mut out = String::new();
    child
        .stdout
        .take()
        .ok_or_else(|| std::io::Error::other("Cannot open stdout"))?
        .read_to_string(&mut out)?;
    print!("{out}");
    child.wait()?;
    Ok(())
}

fn exec_and_args(cmd: &CommandSpec, ctx: &ShellState) -> (PathBuf, Vec<String>) {
    match cmd.command_type() {
        Builtin | Unrecognized => (
            env::current_exe().unwrap(),
            std::iter::once(cmd.name().to_string())
                .chain(cmd.args(ctx))
                .collect(),
        ),
        Executable(path_buf) => (path_buf, cmd.args(ctx)),
    }
}
