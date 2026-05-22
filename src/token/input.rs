use crate::{shell_state::ShellState, token::arg_type::Arg};

use super::input_redirect::InputRedirect;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    pub command: String,
    args: Vec<Arg>,
    pub redirect: Option<InputRedirect>,
}

impl Input {
    pub(crate) fn new(
        command: String,
        args: Vec<Arg>,
        redirect: Option<InputRedirect>,
    ) -> Self {
        Self {
            command,
            args,
            redirect,
        }
    }

    pub fn args(&self, ctx: &ShellState) -> Vec<String> {
        self.args.iter().map(|arg| self.expand_arg(arg, ctx)).collect()
    }

    fn expand_arg(&self, arg: &Arg, ctx: &ShellState) -> String {
        match arg {
            Arg::String(s) => s.clone(),
            Arg::Variable(name) => ctx
                .variables
                .get(name)
                .cloned()
                .unwrap_or_else(|| name.clone()),
            Arg::Concat { parts, .. } => parts
                .iter()
                .map(|part| self.expand_arg(part, ctx))
                .collect(),
        }
    }

    pub fn last_three_elements(&self) -> (&str, Option<&str>, Option<&str>) {
        let mut result = Vec::new();
        result.push(self.command.as_str());
        for arg in &self.args {
            result.push(arg.raw_value());
        }
        if let Some(red) = &self.redirect {
            result.push(">");
            result.push(red.path.as_str());
        }
        (result.pop().unwrap(), result.pop(), result.pop())
    }
}
