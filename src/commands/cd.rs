use std::{
    env::set_current_dir,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use crate::{
    command::{Command, Token},
    command_type::CommandType,
    shell_state::{ShellState, is_executable},
};

pub struct Cd {
    tokens: Vec<Token>,
}

impl Cd {
    pub(super) fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    fn resolve_target(raw: PathBuf, cwd: &Path) -> Result<PathBuf, String> {
        if raw.has_root() {
            return Ok(raw);
        }
        if raw == Path::new("~") {
            return std::env::home_dir()
                .ok_or_else(|| "cd: could not determine home directory".to_string());
        }
        let joined = cwd.join(&raw);
        joined.canonicalize().map_err(|e| {
            let target = joined.display();
            match e.kind() {
                ErrorKind::NotFound => format!("cd: {target}: No such file or directory"),
                ErrorKind::PermissionDenied => format!("cd: {target}: Permission denied"),
                _ => format!("cd: {target}: {e}"),
            }
        })
    }

    fn apply_chdir(path: &Path, ctx: &mut ShellState) -> Result<(), String> {
        if !path.exists() {
            return Err(format!("cd: {}: No such file or directory", path.display()));
        }
        if path.is_file() {
            return Err(format!("cd: not a directory: {}", path.display()));
        }
        let metadata = path
            .metadata()
            .map_err(|e| format!("cd: {}: {e}", path.display()))?;
        if !is_executable(metadata) {
            return Err("permission denied".to_string());
        }
        ctx.cwd = path.to_path_buf();
        set_current_dir(&ctx.cwd).map_err(|e| format!("cd: {}: {e}", path.display()))
    }
}

impl Command for Cd {
    fn execute(&self, ctx: &mut ShellState) {
        self.print(None, None);
        let args = self.args();
        if args.is_empty() {
            return;
        }
        let raw = PathBuf::from(&args[0]);
        let path = match Self::resolve_target(raw, &ctx.cwd) {
            Ok(p) => p,
            Err(msg) => return self.print(None, Some(&msg)),
        };
        if let Err(msg) = Self::apply_chdir(&path, ctx) {
            self.print(None, Some(&msg));
        }
    }

    fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "cd"
    }
}
