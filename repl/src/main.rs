use repl::{Command, ExitCmd, Runnable, UnknownCmd};
use std::io::{self, Write};
use std::process::ExitCode;

fn parse(line: &str) -> Command {
    if line.starts_with("exit") {
        Command::Exit(ExitCmd)
    } else {
        Command::Unknown(UnknownCmd)
    }
}

fn main() -> ExitCode {
    println!("# simple repl  (only valid command is 'exit')");
    println!();

    loop {
        print!("> ");

        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        // println!("#  input: '{}'", input.trim());

        let cmd = parse(&input);
        // println!("#    cmd: '{}'", cmd.name());

        let _ = cmd.run();
        // println!(" # status: {:?}", status);

        if matches!(cmd, Command::Exit(_)) {
            break;
        };
    }

    ExitCode::SUCCESS
}
