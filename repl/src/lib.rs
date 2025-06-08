use std::process::ExitCode;

pub trait Runnable {
    fn run(&self) -> ExitCode;
}

pub struct ExitCmd;

impl Runnable for ExitCmd {
    fn run(&self) -> ExitCode {
        println!("Exiting!");
        println!();
        ExitCode::SUCCESS
    }
}

pub struct UnknownCmd;

impl Runnable for UnknownCmd {
    fn run(&self) -> ExitCode {
        println!("Unknown!");
        println!();
        ExitCode::FAILURE
    }
}
pub enum Command {
    Exit(ExitCmd),
    Unknown(UnknownCmd),
}

impl Command {
    pub fn name(&self) -> &'static str {
        match self {
            Command::Exit(_) => "Exit",
            Command::Unknown(_) => "Unknown",
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
