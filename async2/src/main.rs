// Async search a file
// TODO: add options (pattern and files to search) using clap
// TODO: add logging

use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader};

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
    let pattern = "CRITICAL";
    let path = "/Users/pete/data/afewlogs/0a7e2adc-8bda-443d-a814-60185c7b1c96.log";
    let critical_lines = search_file(pattern, path).await.unwrap();

    println!("# searching for {} in {}", pattern, path);
    dump_lines(critical_lines).await;

    Ok(())
}
