// Async search a file
// TODO: add options (pattern and files to search) using clap
// TODO: add logging

use clap::Parser;
use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[derive(Parser)]
struct Args {
    pattern: String,
    files: Vec<String>,
}

async fn search_file(pattern: &str, path: &str) -> Result<Vec<String>, &'static str> {
    let file = File::open(path).await.map_err(|_| "Could not open file!")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut matching_lines = Vec::new();
    while let Some(line) = lines
        .next_line()
        .await
        .map_err(|_| "failed to read line!")?
    {
        if line.contains(pattern) {
            matching_lines.push(line);
        }
    }
    Ok(matching_lines)
}

async fn dump_lines(lines: Vec<String>) {
    for line in lines {
        println!("{}", line);
    }
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    let pattern = args.pattern;
    for file in args.files {
        let critical_lines = search_file(&pattern, &file).await.unwrap();
        println!("# searching for {} in {}", pattern, file);
        dump_lines(critical_lines).await;
    }

    Ok(())
}
