// Sample code for a filter

use git2::Repository;
use std::collections::HashSet;
use std::path::Path;

trait Filter<T: ?Sized> {
    fn filter(&self, t: &T) -> bool;
}

// AddToGithub - Filter that keeps paths to directories
// containing git repos that haven't yet been uploaded
struct AddToGithub {
    bad_path_components: HashSet<String>,
}

impl AddToGithub {
    fn new<S: AsRef<str>>(list: &[S]) -> Self {
        let bad_path_components = list.iter().map(|s| s.as_ref().to_owned()).collect();
        AddToGithub {
            bad_path_components,
        }
    }
}

// Filter should return true if:
// 1. [IMPLEMENTED] dir is a directory
// 2. [IMPLEMENTED] dir doesn't contain any "bad" path components
//      This lets you avoid dependency checkout paths like the ones under
//      target for rust and Build or .builds for swift/etc.
// 3. [IMPLEMENTED] dir contains a git repo
// 4. [IMPLEMENTED] dir does not have an 'origin' remote
//
// TODO: use regexp matching instead of bad_path_components in step 3
// TODO: dir contains newer commits than origin, and origin is github.com/<username>
impl Filter<Path> for AddToGithub {
    fn filter(&self, path: &Path) -> bool {
        // 1. dir is a directory
        if !path.is_dir() {
            return false;
        }

        // 2. dir doesn't contain any "bad" path components
        if let Some(fname) = path.file_name().and_then(|f| f.to_str()) {
            if self.bad_path_components.contains(fname) {
                return false;
            };
        }

        // 3. dir contains a git repo
        // 4. dir does not have an 'origin' remote
        match Repository::open(path) {
            Ok(repo) => repo.find_remote("origin").is_err(),
            Err(_) => return false,
        }
    }
}

fn main() {
    let filter_dir = AddToGithub::new::<&str>(&[]);

    let dir = Path::new("/Users/pete/practice/rust/size/");
    if filter_dir.filter(&dir) {
        println!("{} should be uploaded", dir.display());
    } else {
        println!("{} should not be uploaded", dir.display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignore_zero_paths() {
        let filter_dir = AddToGithub::new::<&str>(&[]);
        let dir = Path::new("/Users/pete/practice/rust/size");
        assert!(filter_dir.filter(&dir));
    }

    #[test]
    fn test_ignore_one_path() {
        let filter = AddToGithub::new::<&str>(&[".build"]);

        let dir = Path::new("/Users/pete/projects/net/.build");
        assert_eq!(filter.filter(&dir), false);
    }

    #[test]
    fn test_ignore_multiple_paths() {
        let filter = AddToGithub::new::<&str>(&[".build", "target"]);

        let dir = Path::new("/Users/pete/projects/net/.build");
        assert_eq!(filter.filter(&dir), false);
        let dir = Path::new("/Users/pete/projects/net/target");
        assert_eq!(filter.filter(&dir), false);
    }

    #[test]
    fn test_is_not_dir() {
        let filter_dir = AddToGithub::new::<&str>(&[]);
        let bogus_dir = Path::new("/Users/no_such_user/");
        assert_eq!(filter_dir.filter(&bogus_dir), false);
    }

    #[test]
    fn test_is_not_a_repo() {
        let filter_dir = AddToGithub::new::<&str>(&[]);
        let dir = Path::new("/Users/pete/");
        assert_eq!(filter_dir.filter(&dir), false);
    }

    #[test]
    fn test_is_a_repo_with_origin() {
        let filter_dir = AddToGithub::new::<&str>(&[]);
        let dir = Path::new("/Users/pete/practice/practice-rs/");
        assert_eq!(filter_dir.filter(&dir), false);
    }

    #[test]
    fn test_is_a_repo_without_origin() {
        let filter_dir = AddToGithub::new::<&str>(&[]);
        let dir = Path::new("/Users/pete/practice/rust/size/");
        assert!(filter_dir.filter(&dir));
    }
}
