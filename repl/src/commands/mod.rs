mod exit;
mod greet;
mod unknown;
pub use exit::ExitCmd;
pub use greet::GreetCmd;
pub use unknown::UnknownCmd;

use std::process::ExitCode;

pub trait Runnable {
    fn run(&self) -> ExitCode;
}

pub enum Command {
    Exit(ExitCmd),
    Greet(GreetCmd),
    Unknown(UnknownCmd),
}

impl Command {
    // Will use name() later when logging
    // pub fn name(&self) -> &'static str {
    //     match self {
    //         Command::Exit(_) => "exit",
    //         Command::Unknown(_) => "unknown",
    //     }
    // }

    pub fn parse(line: &str) -> Self {
        let trimmed = line.trim();
        let mut tokens = trimmed.split_whitespace();
        let cmd = tokens.next().unwrap_or("").to_ascii_lowercase();
        let args = tokens.collect::<Vec<_>>().join(" ");

        match cmd.as_str() {
            "exit" => Command::Exit(ExitCmd),
            "greet" => Command::Greet(GreetCmd::new(args)),
            _ => Command::Unknown(UnknownCmd),
        }
    }
}

impl Runnable for Command {
    fn run(&self) -> ExitCode {
        match self {
            Command::Exit(e) => e.run(),
            Command::Greet(g) => g.run(),
            Command::Unknown(u) => u.run(),
        }
    }
}
