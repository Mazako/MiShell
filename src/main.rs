#[allow(unused_imports)]
use std::io::{self, Write};


enum CommandType {
    Builtin,
    Executable,
    Unrecognized
}

trait Command {
    fn execute(&self);

    fn args(&self) -> Vec<String>;

    fn command_type(&self) -> CommandType;

    fn name(&self) -> &str;
}

struct Echo {
    args: String,
}

impl Command for Echo {
    fn execute(&self) {
        println!("{}", self.args);
    }

    fn args(&self) -> Vec<String> {
        vec![self.args.clone()]
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }
    
    fn name(&self) -> &str {
        "echo"
    }
}

struct UnknownCommand {
    command: String,
}

impl Command for UnknownCommand {
    fn execute(&self) {
        println!("{}: command not found", self.command);
    }

    fn args(&self) -> Vec<String> {
        vec![]
    }

    fn command_type(&self) -> CommandType {
        CommandType::Unrecognized
    }
    
    fn name(&self) -> &str {
        &self.command
    }
}

struct Exit;

impl Command for Exit {
    fn execute(&self) {
        std::process::exit(0);
    }

    fn args(&self) -> Vec<String> {
        vec![]
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }
    
    fn name(&self) -> &str {
        "exit"
    }
}

struct Type {
    args: Vec<String>
}

impl Command for Type {
    fn execute(&self) {
        let args = self.args();
        if args.is_empty() {
            println!();
        }
        let cmd = command_from_input(&args[0]);
        match cmd.command_type() {
            CommandType::Builtin => println!("{} is a shell builtin", cmd.name()),
            CommandType::Executable => print!("{} is a executable", cmd.name()),
            CommandType::Unrecognized => println!("{}: not found", cmd.name()),
        }
    }

    fn args(&self) -> Vec<String> {
        self.args.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }
    
    fn name(&self) -> &str {
        "type"
    }
}


fn command_args(command: &str) -> (&str, &str) {
    let command = command.trim();
    if let Some(idx) = command.find(" ") {
        let args = &command[idx + 1..].trim();
        // let args = command[idx + 1..].split_whitespace().map(|s| s.to_string()).collect();
        return (&command[..idx], args);
    }
    (command, "")
}

fn whitespace_args(args: &str) -> Vec<String> {
    args.split_whitespace().map(|s| s.to_string()).collect()
}

fn command_from_input(input: &str) -> Box<dyn Command> {
    let (command, args) = command_args(input);
    match command {
        "echo" => Box::new(Echo { args: args.to_string() }),
        "exit" => Box::new(Exit),
        "type" => Box::new(Type {args: whitespace_args(args)}),
        _ => Box::new(UnknownCommand { command: command.to_string() }),
    }
}

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command_from_input(command.trim());
        command.execute();
    }
}
