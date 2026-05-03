use std::path::PathBuf;

pub enum CommandType {
    Builtin,
    Executable(PathBuf),
    Unrecognized,
}
