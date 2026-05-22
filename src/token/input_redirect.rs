use std::fs::{self, File, OpenOptions};
use std::path::Path;

use super::redirect_target::RedirectTarget;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InputRedirect {
    pub target: RedirectTarget,
    pub path: String,
    pub append: bool,
}

impl InputRedirect {
    pub fn open_write(&self) -> File {
        if let Some(parent) = Path::new(&self.path).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).unwrap();
            }
        }
        let mut opts = OpenOptions::new();
        opts.create(true).write(true);
        if self.append {
            opts.append(true);
        } else {
            opts.truncate(true);
        }
        opts.open(&self.path).unwrap()
    }

    pub fn touch(&self) {
        let _ = self.open_write();
    }
}
