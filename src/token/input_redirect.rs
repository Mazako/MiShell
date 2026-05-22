use std::fs::{File, OpenOptions};

use super::redirect_target::RedirectTarget;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InputRedirect {
    pub target: RedirectTarget,
    pub path: String,
    pub append: bool,
}

impl InputRedirect {
    pub fn open_write(&self) -> File {
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
