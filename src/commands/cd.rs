use std::{
    env::set_current_dir,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use crate::{
    command_io, shell_state::{ShellState, is_executable}, token::Input,
};

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

pub fn run(input: &Input, ctx: &mut ShellState) {
    input.touch_redirects();
    let args = input.args(ctx);
    if args.is_empty() {
        return;
    }
    let raw = PathBuf::from(&args[0]);
    let path = match resolve_target(raw, &ctx.cwd) {
        Ok(p) => p,
        Err(msg) => return command_io::print_stderr(input, &msg),
    };
    if let Err(msg) = apply_chdir(&path, ctx) {
        command_io::print_stderr(input, &msg);
    }
}
