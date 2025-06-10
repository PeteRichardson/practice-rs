use crate::commands::Runnable;
use std::process::ExitCode;

pub struct UnknownCmd;

impl Runnable for UnknownCmd {
    fn run(&self) -> ExitCode {
        println!("Unknown!");
        println!();
        ExitCode::FAILURE
    }
}
