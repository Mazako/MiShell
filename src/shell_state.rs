use std::path::PathBuf;

pub struct ShellState {
    pub path_dirs: Vec<PathBuf>,
}

impl ShellState {
    pub fn new() -> Self {
        let path_dirs = std::env::var("PATH")
            .map(|p| p.split(':').map(PathBuf::from).collect())
            .unwrap_or_default();
        Self { path_dirs }
    }
}
