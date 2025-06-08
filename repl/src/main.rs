use repl::{Command, ExitCmd, Runnable, UnknownCmd};
use rustyline::config::{Config, EditMode};
use rustyline::{DefaultEditor, Result};
use std::process::ExitCode;

fn parse(line: &str) -> Command {
    if line.starts_with("exit") {
        Command::Exit(ExitCmd)
    } else {
        Command::Unknown(UnknownCmd)
    }
}

fn main() -> Result<ExitCode> {
    println!("# simple repl  (only valid command is 'exit')");
    println!();

    let config = Config::builder().edit_mode(EditMode::Vi).build();
    let mut rl = DefaultEditor::with_config(config)?;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let cmd = parse(&line);
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
