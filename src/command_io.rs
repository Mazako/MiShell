use std::io::Write;
use std::process::{Command, Stdio};

use crate::token::{Input, InputRedirect, RedirectTarget};

fn write(text: Option<&str>, redirect: Option<&InputRedirect>, inherit: impl FnOnce(&str)) {
    match (text, redirect) {
        (Some(s), Some(r)) => writeln!(r.open_write(), "{s}").unwrap(),
        (Some(s), None) => inherit(s),
        (None, Some(r)) => r.touch(),
        (None, None) => {}
    }
}

pub fn print_stdout(input: &Input, s: &str) {
    write(
        Some(s),
        input.redirect_for(RedirectTarget::Stdout),
        |s| println!("{s}"),
    );
    touch_redirect_if_unused(input, RedirectTarget::Stderr);
}

pub fn print_stderr(input: &Input, s: &str) {
    write(
        Some(s),
        input.redirect_for(RedirectTarget::Stderr),
        |s| eprintln!("{s}"),
    );
    touch_redirect_if_unused(input, RedirectTarget::Stdout);
}

/// Creates redirect target file when this stream has no output (e.g. `echo … 2> file`).
fn touch_redirect_if_unused(input: &Input, target: RedirectTarget) {
    if let Some(r) = input.redirect_for(target) {
        r.touch();
    }
}

pub fn apply_redirects(input: &Input, command: &mut Command) {
    if let Some(r) = input.redirect_for(RedirectTarget::Stdout) {
        command.stdout(Stdio::from(r.open_write()));
    }
    if let Some(r) = input.redirect_for(RedirectTarget::Stderr) {
        command.stderr(Stdio::from(r.open_write()));
    }
}
