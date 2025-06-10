use crate::commands::Runnable;
use std::process::ExitCode;

pub struct ExitCmd;

impl Runnable for ExitCmd {
    fn run(&self) -> ExitCode {
        println!("Exiting!");
        println!();
        ExitCode::SUCCESS
    }
}
