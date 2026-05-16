use std::{
    borrow::Cow,
    cell::RefCell,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
    rc::Rc,
};

use rustyline::{
    Result,
    history::{History, SearchDirection, SearchResult},
};

#[derive(Default)]
pub struct ShellHistory {
    pub history: Vec<String>,
    last_append_idx: usize
}

impl ShellHistory {
    pub fn read_from_file(&mut self, path: &Path) -> std::result::Result<(), String> {
        let contents = fs::read_to_string(path).map_err(map_io_error(path))?;
        let lines: Vec<String> = contents.lines().map(str::to_string).collect();
        self.history.extend(lines);
        Ok(())
    }

    pub fn write_to_file(&self, path: &Path) -> std::result::Result<(), String> {
        let mut result = self.history.join("\n");
        result.push('\n');
        fs::write(path, result).map_err(map_io_error(path))
    }

    pub fn append_to_file(&mut self, path: &Path) -> std::result::Result<(), String> {
        if self.last_append_idx == self.history.len() {
            return Ok(());
        }
        let hist_slize = &self.history[self.last_append_idx..];
        let mut result = hist_slize.join("\n");
        result.push('\n');
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .map_err(map_io_error(path))?;
        let _ = file.write_all(result.as_bytes()).map_err(map_io_error(path));
        self.last_append_idx = self.history.len();
        Ok(())
    }
}

fn map_io_error(path: &Path) -> impl FnOnce(io::Error) -> String {
    |e| match e.kind() {
        io::ErrorKind::NotFound => {
            format!("history: {}: No such file or directory", path.display())
        }
        io::ErrorKind::PermissionDenied => {
            format!("history: {}: Permission denied", path.display())
        }
        _ => format!("history: {}: {e}", path.display()),
    }
}

pub struct SharedShellHistory(pub Rc<RefCell<ShellHistory>>);

impl History for SharedShellHistory {
    fn get(&self, index: usize, _dir: SearchDirection) -> Result<Option<SearchResult<'_>>> {
        let inner = self.0.borrow();
        let Some(line) = inner.history.get(index) else {
            return Ok(None);
        };
        Ok(Some(SearchResult {
            entry: Cow::Owned(line.clone()),
            pos: 0,
            idx: index,
        }))
    }

    fn add(&mut self, line: &str) -> Result<bool> {
        self.0.borrow_mut().history.push(line.to_string());
        Ok(true)
    }

    fn add_owned(&mut self, line: String) -> Result<bool> {
        self.0.borrow_mut().history.push(line);
        Ok(true)
    }

    fn len(&self) -> usize {
        self.0.borrow().history.len()
    }

    fn is_empty(&self) -> bool {
        self.0.borrow().history.is_empty()
    }

    fn set_max_len(&mut self, _len: usize) -> Result<()> {
        Ok(())
    }

    fn ignore_dups(&mut self, _yes: bool) -> Result<()> {
        Ok(())
    }

    fn ignore_space(&mut self, _yes: bool) {}

    fn save(&mut self, _path: &Path) -> Result<()> {
        Ok(())
    }

    fn append(&mut self, _path: &Path) -> Result<()> {
        Ok(())
    }

    fn load(&mut self, _path: &Path) -> Result<()> {
        Ok(())
    }

    fn clear(&mut self) -> Result<()> {
        self.0.borrow_mut().history.clear();
        Ok(())
    }

    fn search(
        &self,
        _term: &str,
        _start: usize,
        _dir: SearchDirection,
    ) -> Result<Option<SearchResult<'_>>> {
        Ok(None)
    }

    fn starts_with(
        &self,
        _term: &str,
        _start: usize,
        _dir: SearchDirection,
    ) -> Result<Option<SearchResult<'_>>> {
        Ok(None)
    }
}
