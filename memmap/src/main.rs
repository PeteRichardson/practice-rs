use clap::Parser;
use glob::glob;
use log::debug;
use std::fs::File;
use std::io::BufRead;

use memmap2::Mmap;

#[derive(Parser, Debug)]
struct Args {
    filename_patterns: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Args::parse();

    for pattern in args.filename_patterns {
        debug!("# handling pattern '{}'", &pattern);
        for entry in glob(&pattern)? {
            debug!("# handling entry '{:?}'", &entry);
            match entry {
                Ok(path) => {
                    let file = File::open(&path)?;

                    let mmap = unsafe { Mmap::map(&file)? };
                    let mut lines = mmap[..].lines();
                    let count = lines.try_fold(0, |acc, line| line.map(|_| acc + 1))?;

                    println!("{:8}  {}", count, &path.display());
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }

    Ok(())
}
