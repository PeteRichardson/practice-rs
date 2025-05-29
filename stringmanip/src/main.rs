// Experimental code to simplify a .git directory path that is known
// to be a subdirectory of some base path.  (e.g. the path is
// found as part of a walk_dir from the base)
//
// The simplifications are:
// 1. remove the .git at the end of the path
// 2. remove the base components from the front of the path
//
// e.g. base:  /Users/pete/projects
//      path:  /Users/pete/projects/libraries/foo/.git
// should return:   libraries/foo
//
// It will be used to reduce unneeded text in the output.
// i.e. ".git" is not needed, and all paths have base
// as the prefix, so it can be safely removed.

use anyhow::Result;
use std::path::Path;

fn dump(label: &str, path: &Path) {
    println!("{}: {}", label, path.display());
}

fn simplified_repo_path(path: &Path, base: &Path) -> String {
    if let Some(display_path) = path.parent().and_then(|p| p.strip_prefix(base).ok()) {
        return display_path.display().to_string();
    } else {
        // should never happen, since base is the root dir where the walk starts,
        // so all paths will have a parent and be under base.
        panic!(
            "Impossible! path ({}) has no parent or is not under base ({})!",
            path.display(),
            base.display()
        );
    };
}

fn main() -> Result<()> {
    let base = Path::new("/Users/pete/practice/");
    assert!(base.exists(), "base {} doesn't exist", base.display());
    dump("base", base);

    let path = Path::new("/Users/pete/practice/objc/people/.git");
    assert!(path.exists(), "path {} doesn't exist", path.display());
    dump("path", path);

    let simple_path = simplified_repo_path(&path, &base);
    dump("display_path", Path::new(&simple_path));

    Ok(())
}
