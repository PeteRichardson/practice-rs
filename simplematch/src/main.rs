use std::error::Error;
use std::process::ExitCode;

fn run(i: u8) -> Result<ExitCode, Box<dyn Error>> {
    match i {
        0..=3 => Err("something 3 failed but I don't care!".into()),
        4..=50 => {
            println!("Yay! it works! {}", i);
            Ok(ExitCode::SUCCESS)
        }
        51..=150 => {
            println!("Booya! it works! {}", i);
            Ok(ExitCode::SUCCESS)
        }
        151..=199 => {
            println!("Wowza! it works! {}", i);
            Ok(ExitCode::SUCCESS)
        }
        _ => {
            println!("Huh? Too big! {}", i);
            Err(format!("Something's not right. {}", i).into())
        }
    }
}

fn main() -> ExitCode {
    let x = rand::random::<u8>();
    run(x).unwrap_or_else(|err| {
        eprintln!("Unexpected error: {}", err);
        ExitCode::FAILURE
    })
} 
