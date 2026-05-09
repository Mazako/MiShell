mod tokenizer;

use std::fs::{File, OpenOptions};

pub use tokenizer::parse_input;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RedirectTarget {
    Stderr,
    Stdout,
}

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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    pub command: String,
    pub args: Vec<String>,
    pub redirect: Option<InputRedirect>,
}

impl Input {
    pub fn last_three_elements(&self) -> (&str, Option<&str>, Option<&str> ){
        let mut result = Vec::new();
        result.push(self.command.as_str());
        let args: Vec<&str> = self.args.iter().map(|f| f.as_str()).collect();
        result.extend_from_slice(&args);
        if let Some(red) = &self.redirect {
            result.push(">");
            result.push(red.path.as_str());
        }
        (result.pop().unwrap(), result.pop(), result.pop())
    }
}
