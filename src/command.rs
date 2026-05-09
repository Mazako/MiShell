use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::Stdio;

use crate::command_type::CommandType;
use crate::shell_state::ShellState;
use crate::token::{Input, InputRedirect, RedirectTarget};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StreamTarget {
    Inherit,
    Redirect(InputRedirect),
}

pub trait Command {
    fn input(&self) -> Input;

    fn execute(&self, ctx: &mut ShellState);

    fn args(&self) -> Vec<String> {
        self.input().args
    }

    fn stdout(&self) -> StreamTarget {
        if let Some(red) = self.input().redirect
            && red.target == RedirectTarget::Stdout
        {
            StreamTarget::Redirect(red)
        } else {
            StreamTarget::Inherit
        }
    }

    fn stderr(&self) -> StreamTarget {
        if let Some(red) = self.input().redirect
            && red.target == RedirectTarget::Stderr
        {
            StreamTarget::Redirect(red)
        } else {
            StreamTarget::Inherit
        }
    }

    fn print(&self, stdout: Option<&str>, stderr: Option<&str>) {
        match (stdout, self.stdout()) {
            (Some(stdout), StreamTarget::Redirect(r)) => {
                writeln!(r.open_write(), "{stdout}").unwrap()
            }
            (Some(stdout), StreamTarget::Inherit) => println!("{stdout}"),
            (None, StreamTarget::Redirect(r)) => r.touch(),
            (None, StreamTarget::Inherit) => {}
        }
        match (stderr, self.stderr()) {
            (Some(stderr), StreamTarget::Redirect(r)) => {
                writeln!(r.open_write(), "{stderr}").unwrap()
            }
            (Some(stderr), StreamTarget::Inherit) => eprintln!("{stderr}"),
            (None, StreamTarget::Redirect(r)) => r.touch(),
            (None, StreamTarget::Inherit) => {}
        }
    }

    fn apply_redirects(&self, command: &mut std::process::Command) {
        if let StreamTarget::Redirect(r) = self.stdout() {
            command.stdout(Stdio::from(r.open_write()));
        }
        if let StreamTarget::Redirect(r) = self.stderr() {
            command.stderr(Stdio::from(r.open_write()));
        }
    }

    fn command_type(&self) -> CommandType;

    fn name(&self) -> &str;
}
