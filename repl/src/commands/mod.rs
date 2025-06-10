mod exit;
mod unknown;
pub use exit::ExitCmd;
pub use unknown::UnknownCmd;

use std::process::ExitCode;

pub trait Runnable {
    fn run(&self) -> ExitCode;
}

pub enum Command {
    Exit(ExitCmd),
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
        if line.trim().eq_ignore_ascii_case("exit") {
            Command::Exit(ExitCmd)
        } else {
            Command::Unknown(UnknownCmd)
        }
    }
}

impl Runnable for Command {
    fn run(&self) -> ExitCode {
        match self {
            Command::Exit(e) => e.run(),
            Command::Unknown(u) => u.run(),
        }
    }
}
