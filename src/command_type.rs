use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum CommandType {
    Builtin,
    Executable(PathBuf),
    Unrecognized,
}
