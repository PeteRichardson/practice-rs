use async_std::task;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    // Starting Directory (default ".")
    #[arg(short = 'd', long = "dir", default_value = ".")]
    dir: PathBuf,
}

async fn walk_dir(dir: PathBuf) {
    println!("Hello, {}!", dir.display());
}

#[async_std::main]
async fn main() {
    let args = Args::parse();
    let root_dir = args.dir;

    let handle = task::spawn(async {
        walk_dir(root_dir).await;
    });
    handle.await;
}
