mod commands;
use commands::{Command, Runnable};
use rustyline::config::{Config, EditMode};
use rustyline::{DefaultEditor, Result};
use std::process::ExitCode;

fn main() -> Result<ExitCode> {
    println!("# simple repl  (only valid command is 'exit')");
    println!();

    let config = Config::builder().edit_mode(EditMode::Vi).build();
    let mut rl = DefaultEditor::with_config(config)?;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let cmd = Command::parse(&line);
                let _ = cmd.run();
                if matches!(cmd, Command::Exit(_)) {
                    break;
                };
            }
            Err(_) => println!("No input"),
        }
    }

    Ok(ExitCode::SUCCESS)
}
