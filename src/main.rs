mod command;
mod command_type;
mod commands;

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Arc;

use commands::command_from_input;

fn main() {
    let execs = init_path();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command_from_input(command.trim(), &execs);
        command.execute();
    }
}


pub fn init_path() -> Arc<HashMap<String, PathBuf>> {
    let mut execs: HashMap<String, PathBuf> = HashMap::new();
    let path = std::env::var("PATH").unwrap();
    for ele in path.split(":") {
        if let Ok(res) = fs::read_dir(ele) {
            for e in res.flatten() {
                if let Ok(file_type) = e.file_type() && file_type.is_file()
                    && let Ok(name) = e.file_name().into_string() {
                        if !execs.contains_key(&name) {
                            execs.insert(name, e.path());
                        }
                    }
            }
        }
    }
    Arc::new(execs)
}
