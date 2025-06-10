use crate::commands::Runnable;
use std::process::ExitCode;

pub struct GreetCmd {
    name: String,
}

impl GreetCmd {
    pub fn new(args: String) -> Self {
        GreetCmd {
            name: args
                .split_ascii_whitespace()
                .next()
                .unwrap_or("mystery person")
                .to_string(),
        }
    }
}

impl Runnable for GreetCmd {
    fn run(&self) -> ExitCode {
        println!("Hello, {}!", self.name);
        println!();
        ExitCode::SUCCESS
    }
}
