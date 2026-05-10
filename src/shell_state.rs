use std::{
    collections::HashMap,
    fs::{self, Metadata},
    os::unix::fs::PermissionsExt,
    path::PathBuf, process::Child,
};

use indexmap::IndexMap;

pub struct ShellState {
    pub path_dirs: Vec<PathBuf>,
    pub cwd: PathBuf,
    path_commands: HashMap<String, PathBuf>,
    completions_scripts: HashMap<String, PathBuf>,
    background_processes: IndexMap<u32, (Child, String)>,
    id_generator: u32
}

impl ShellState {
    pub fn new() -> Self {
        let path_dirs = std::env::var("PATH")
            .map(|p| p.split(':').map(PathBuf::from).collect())
            .unwrap_or_default();
        let cwd = std::env::current_dir().unwrap();
        let path_commands = collect_path_execs(&path_dirs);
        Self {
            path_dirs,
            cwd,
            path_commands,
            completions_scripts: HashMap::new(),
            background_processes: IndexMap::new(),
            id_generator: 1
        }
    }

    pub fn find_in_path(&self, command: &str) -> Option<PathBuf> {
        self.path_commands.get(command).cloned()
    }

    pub fn path_command_names(&self) -> Vec<&String> {
        self.path_commands.keys().collect()
    }

    pub fn add_completion_script(&mut self, command: &str, path: PathBuf) {
        self.completions_scripts.insert(command.to_string(), path);
    }

    pub fn completion_script_for(&self, command: &str) -> Option<&PathBuf> {
        self.completions_scripts.get(command)
    }

    pub fn remove_completion_script(&mut self, command: &str) {
        self.completions_scripts.remove(command);
    }

    pub fn add_child(&mut self, child: Child, line: &str) -> (u32, u32) {
        let pid = child.id();
        let id = self.id_generator;
        self.background_processes.insert(id, (child, line.to_string()));
        self.id_generator += 1;
        (id, pid)
    }

    pub fn print_background_jobs(&mut self) {
        let mut lines: Vec<String> = Vec::new();
        let len = self.background_processes.len();
        for (i, (id, (child, path))) in self.background_processes.iter_mut().enumerate() {
            let mut line = String::new();
            line.push_str(&format!("[{id}]"));
            if i == len - 1 {
                line.push_str("+ ");
            } else if i == len - 2 {
                line.push_str("- ");
            } else {
                line.push(' ');
            }
            let status_str = match child.try_wait() {
                Ok(Some(_)) => format!("{:24}", "Done"),
                Ok(None) => format!("{:24}", "Running"),
                Err(_) => "".to_string(),
            };
            line.push_str(&status_str);
            line.push_str(path);
            lines.push(line);
        }
        for ele in lines {
            println!("{ele}");
        }
    }

}

pub fn is_executable(metadata: Metadata) -> bool {
    metadata.permissions().mode() & 0o111 != 0
}

fn collect_path_execs(path_dirs: &Vec<PathBuf>) -> HashMap<String, PathBuf> {
    let mut map: HashMap<String, PathBuf> = HashMap::new();
    for dir in path_dirs {
        if let Ok(res) = fs::read_dir(dir) {
            for e in res.flatten() {
                if let Ok(metadata) = e.metadata()
                    && metadata.is_file()
                    && let Ok(name) = e.file_name().into_string()
                    && is_executable(metadata)
                    && !map.contains_key(&name)
                {
                    map.insert(name, e.path());
                }
            }
        }
    }
    map
}
