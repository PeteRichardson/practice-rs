use clap::Parser;
use glob::glob;
use log::debug;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use threadpool::ThreadPool;

use memmap2::Mmap;

#[derive(Parser, Debug)]
struct Args {
    /// filename patterns to count  (e.g. 'data/*.json')
    filename_patterns: Vec<String>,
}

/// count lines in file by:
/// 1. memory mapping file
/// 2. calling lines() on the whole slice
/// 3. iterating and accumulating the count
fn count_lines(path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let mut lines = mmap[..].lines();
    let count = lines.try_fold(0, |acc, line| line.map(|_| acc + 1))?;
    Ok(count)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Args::parse();
    let pool = ThreadPool::new(16);

    for pattern in args.filename_patterns {
        debug!("# handling pattern '{}'", &pattern);
        for entry in glob(&pattern)? {
            debug!("# handling entry '{:?}'", &entry);
            match entry {
                Ok(path) => pool.execute(move || {
                    let count: usize = count_lines(&path).expect("Couldn't count lines!");
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
        assert_eq!(
            count_lines("data/AliceInWonderland.txt".as_ref()).unwrap(),
            24
        );
    }
}
