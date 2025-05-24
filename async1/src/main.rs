use anyhow::Context;
use async_std::fs;
use async_std::path::PathBuf;
use async_std::prelude::*;
use async_std::task;
use clap::Parser;

#[derive(Parser)]
struct Args {
    // Starting Directory (default ".")
    #[arg(short = 'd', long = "dir", default_value = ".")]
    dir: PathBuf,
}

async fn walk_dir(dir: PathBuf) -> anyhow::Result<()> {
    let mut entries = fs::read_dir(&dir)
        .await
        .with_context(|| format!("Failed to read directory: {}", dir.display()))?;

    while let Some(entry_res) = entries
        .next()
        .await
        .transpose()
        .with_context(|| format!("Failed to read entry in {}", dir.display()))?
    {
        let path = entry_res.path();
        println!("# Checking dir: {}", path.display());
    }

    Ok(())
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let root_dir = args.dir;

    let handle = task::spawn(async {
        let _ = walk_dir(root_dir).await;
    });
    handle.await;

    Ok(())
}
