use std::{
    cmp::Reverse, collections::{BinaryHeap, HashMap}, fs::{self, Metadata}, io, os::unix::fs::PermissionsExt, path::PathBuf, process::{Child, ExitStatus}
};

use indexmap::IndexMap;
use rustyline::history;

const JOB_STATUS_WIDTH: usize = 21;

enum JobWaitOutcome {
    Show { status: &'static str, reap: bool },
    Skip,
}

fn interpret_job_wait(result: io::Result<Option<ExitStatus>>, done_only: bool) -> JobWaitOutcome {
    match result {
        Ok(Some(_)) => JobWaitOutcome::Show {
            status: "Done",
            reap: true,
        },
        Ok(None) if done_only => JobWaitOutcome::Skip,
        Ok(None) => JobWaitOutcome::Show {
            status: "Running",
            reap: false,
        },
        Err(_) if done_only => JobWaitOutcome::Skip,
        Err(_) => JobWaitOutcome::Show {
            status: "",
            reap: false,
        },
    }
}

fn job_list_marker(index: usize, total: usize) -> &'static str {
    if index + 1 == total {
        "+  "
    } else if index + 2 == total {
        "-  "
    } else {
        "  "
    }
}

fn format_job_row(id: u32, marker: &str, status: &str, command: &str) -> String {
    format!(
        "[{id}]{marker}{status:<w$}{command}",
        w = JOB_STATUS_WIDTH
    )
}

pub struct ShellState {
    pub path_dirs: Vec<PathBuf>,
    pub cwd: PathBuf,
    path_commands: HashMap<String, PathBuf>,
    completions_scripts: HashMap<String, PathBuf>,
    background_processes: IndexMap<u32, (Child, String)>,
    available_ids: BinaryHeap<Reverse<u32>>,
    id_generator: u32,
    pub history: Vec<String>
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
            available_ids: BinaryHeap::new(),
            id_generator: 1,
            history: Vec::new()
        }
    }

    fn background_id(&mut self) -> u32 {
        if let Some(Reverse(id)) = self.available_ids.pop() {
            id
        } else {
            let new_id = self.id_generator;
            self.id_generator += 1;
            new_id
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
        let id = self.background_id();
        let cmd_line = line
            .trim_end()
            .trim_end_matches('&')
            .trim_end()
            .to_string();
        self.background_processes
            .insert(id, (child, cmd_line));
        (id, pid)
    }

    fn remove_childs(&mut self, ids: &[u32]) {
        ids.iter().for_each(|id| {
            self.available_ids.push(Reverse(*id));
            self.background_processes.shift_remove(id);
        });
    }

    pub fn print_and_reap(&mut self, done_only: bool) {
        let total = self.background_processes.len();
        let mut done_ids = Vec::new();
        let mut lines = Vec::new();

        for (i, (id, (child, command))) in self.background_processes.iter_mut().enumerate() {
            let JobWaitOutcome::Show { status, reap } =
                interpret_job_wait(child.try_wait(), done_only)
            else {
                continue;
            };
            if reap {
                done_ids.push(*id);
            }
            lines.push(format_job_row(*id, job_list_marker(i, total), status, command));
        }

        for line in lines {
            println!("{line}");
        }
        self.remove_childs(&done_ids);
    }

    pub fn add_to_history(&mut self, line: &str) {
        self.history.push(line.to_string());
    }

    pub fn history_last_n(&self, n: usize) -> &[String] {
        &self.history[self.history.len()-n..]
    }

    pub fn history_len(&self) -> usize {
        self.history.len()
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