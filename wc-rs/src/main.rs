use clap::Parser;
use glob::glob;
use log::debug;
use std::fs::File;
use std::io::BufRead;
use std::io::{stdin, BufReader, Read};
use threadpool::ThreadPool;

use memmap2::Mmap;

#[derive(Parser, Debug)]
struct Args {
    /// filename patterns to count  (e.g. 'data/*.json')
    filename_patterns: Vec<String>,
}

fn count_lines_from_bytes(bytes: &[u8]) -> usize {
    bytes
        .lines()
        .try_fold(0, |acc: usize, line| line.map(|_| acc + 1))
        .expect("couldn't fold")
}

fn count_stdin() -> Result<usize, std::io::Error> {
    let mut input: String = Default::default();
    let stdin = stdin().lock();
    let _ = BufReader::new(stdin).read_to_string(&mut input)?;
    Ok(count_lines_from_bytes(input.as_bytes()))
}

fn count_file(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    Ok(count_lines_from_bytes(&mmap[..]))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Args::parse();
    let pool = ThreadPool::new(16);

    for pattern in args.filename_patterns {
        debug!("# handling pattern '{}'", &pattern);
        if pattern == "-" {
            let count: usize = count_stdin().expect("problem reading stdin");
            println!("{:8} stdin", count);
            continue;
        }
        for entry in glob(&pattern)? {
            debug!("# handling entry '{:?}'", &entry);
            match entry {
                Ok(path) => pool.execute(move || {
                    let count: usize =
                        count_file(&path.to_string_lossy()).expect("Couldn't count lines!");
                    println!("{:8} {}", count, path.display());
                }),
                Err(e) => println!("{:?}", e),
            }
        }
    }
    pool.join();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines() {
        assert_eq!(count_file("data/AliceInWonderland.txt").unwrap(), 24);
    }
}
