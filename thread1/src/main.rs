use clap::Parser;
use log::{debug, info};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::{self, JoinHandle};

fn count_lines(path: &str) -> Result<usize, std::io::Error> {
    info!("{:?}: counting '{}'", thread::current().id(), path);
    let mut lines = BufReader::new(File::open(path)?).lines();
    let count = lines.try_fold(0, |acc, line| line.map(|_| acc + 1))?;
    Ok(count)
}
#[derive(Parser, Debug)]
struct Args {
    paths: Vec<String>,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    debug!("Main thread starting");
    let mut threads: Vec<JoinHandle<()>> = vec![];
    for path in args.paths {
        threads.push(thread::spawn(move || {
            let count: usize = count_lines(&path).expect("Couldn't count lines!");
            println!("{} - {} lines", path, count);
        }))
    }

    for t in threads {
        t.join().unwrap()
    }
    debug!("Main thread ending");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines() {
        assert_eq!(count_lines("data/target.txt").unwrap(), 6);
    }
}
