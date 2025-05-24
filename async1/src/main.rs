use async_std::fs;
use async_std::path::PathBuf;
use async_std::stream::StreamExt; // 💡 Bring in `.next()`
use async_std::task::{self, JoinHandle};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use clap::Parser;

/// Walk a directory tree asynchronously.
#[derive(Parser)]
struct Args {
    /// Directory to start walking from (default: ".")
    #[arg(short = 'd', long = "dir", default_value = ".")]
    dir: PathBuf,
}

#[async_std::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let root_dir = args.dir;

    // Shared list of JoinHandles (thread-safe and no need for async locking).
    let tasks: Arc<Mutex<Vec<JoinHandle<()>>>> = Arc::new(Mutex::new(Vec::new()));

    // Spawn the initial task to walk the root directory.
    let tasks_clone = tasks.clone();
    let initial_task = task::spawn(async move {
        if let Err(e) = walk_dir(root_dir, tasks_clone).await {
            eprintln!("Error in root: {:?}", e);
        }
    });

    tasks.lock().unwrap().push(initial_task);

    // Wait for all tasks to complete.
    loop {
        let current_tasks = {
            let mut locked = tasks.lock().unwrap();
            if locked.is_empty() {
                break;
            }
            std::mem::take(&mut *locked)
        };

        for handle in current_tasks {
            handle.await;
        }
    }

    Ok(())
}

fn walk_dir(
    dir: PathBuf,
    tasks: Arc<Mutex<Vec<JoinHandle<()>>>>,
) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
    Box::pin(async move {
        let mut entries = fs::read_dir(&dir)
            .await
            .with_context(|| format!("Failed to read directory: {}", dir.display()))?;

        while let Some(entry) = entries
            .next()
            .await
            .transpose()
            .with_context(|| format!("Failed to read entry in {}", dir.display()))?
        {
            let path = entry.path();
            let file_type = entry
                .file_type()
                .await
                .with_context(|| format!("Failed to get file type for {}", path.display()))?;

            if file_type.is_dir() {
                let tasks_clone = tasks.clone();
                let path_clone = path.clone();
                let new_task = task::spawn(async move {
                    if let Err(e) = walk_dir(path_clone, tasks_clone).await {
                        eprintln!("Error in {}: {:?}", path.display(), e);
                    }
                });
                tasks.lock().unwrap().push(new_task);
            } else if file_type.is_file() {
                println!("File: {}", path.display());
            }
        }

        Ok(())
    })
}
