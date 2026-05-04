use std::path::PathBuf;

pub struct ShellState {
    pub path_dirs: Vec<PathBuf>,
    pub cwd:  PathBuf
}

impl ShellState {
    pub fn new() -> Self {
        let path_dirs = std::env::var("PATH")
            .map(|p| p.split(':').map(PathBuf::from).collect())
            .unwrap_or_default();
        let cwd = std::env::current_dir().unwrap();
        Self { path_dirs, cwd }
    }
}
