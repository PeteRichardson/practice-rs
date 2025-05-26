use filter::{AddToGithub, Filter};
use std::path::Path;

fn main() {
    let filter_dir = AddToGithub::new::<&str>(&[]);

    let dir = Path::new("/Users/pete/practice/rust/size/");
    if filter_dir.filter(&dir) {
        println!("{} should be uploaded", dir.display());
    } else {
        println!("{} should not be uploaded", dir.display());
    }
}
