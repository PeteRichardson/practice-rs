use clap::Parser;
use log::debug;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
struct Args {
    /// directory to list
    #[arg(default_value = ".")]
    directory: PathBuf,
}

fn main() {
    env_logger::init();

    let args: Args = Args::parse();
    debug!("# listing directory {}", args.directory.display());

    let lslines = Command::new("ls")
        .arg("-la")
        .arg(&args.directory)
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&lslines.stdout);
    if output.is_empty() {
        println!("# No files found in {}", args.directory.display());
    } else {
        println!("{}", output)
    };
}
