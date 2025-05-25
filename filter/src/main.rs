// Sample code for a filter

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
// 3. [NOT IMPLEMENTED YET] dir contains a git repo
// 4. [NOT IMPLEMENTED YET] dir has no 'origin' remote
//
// TODO: use regexp matching instead of bad_path_components in step 3
// TODO: dir contains newer commits than origin, and origin is github.com/<username>
impl Filter<Path> for AddToGithub {
    fn filter(&self, path: &Path) -> bool {
        if !path.is_dir() {
            return false;
        }

        if let Some(fname) = path.file_name().and_then(|f| f.to_str()) {
            !self.bad_path_components.contains(fname)
        } else {
            false
        }
    }
}

fn main() {
    let filter_dir = AddToGithub::new::<&str>(&[]);

    let dir = Path::new("https://github.com/PeteRichardson/net");
    assert!(filter_dir.filter(&dir));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignore_zero_paths() {
        let filter_dir = AddToGithub::new::<&str>(&[]);
        let dir = Path::new("/Users/pete/projects/net");
        assert!(filter_dir.filter(&dir));
    }

    #[test]
    fn test_ignore_one_path() {
        let filter = AddToGithub::new::<&str>(&[".build"]);

        let dir = Path::new("/Users/pete/projects/net/.build");
        assert_eq!(filter.filter(&dir), false);
        let dir = Path::new("/Users/pete/projects/net");
        assert!(filter.filter(&dir));
    }

    #[test]
    fn test_ignore_multiple_paths() {
        let filter = AddToGithub::new::<&str>(&[".build", "target"]);

        let dir = Path::new("/Users/pete/projects/net/.build");
        assert_eq!(filter.filter(&dir), false);
        let dir = Path::new("/Users/pete/projects/net/target");
        assert_eq!(filter.filter(&dir), false);
        let dir = Path::new("/Users/pete/projects/net");
        assert_eq!(filter.filter(&dir), true);
    }

    #[test]
    fn test_is_dir() {
        let filter_dir = AddToGithub::new::<&str>(&[]);
        let dir = Path::new("/Users/pete/");
        assert!(filter_dir.filter(&dir));
    }

    #[test]
    fn test_is_not_dir() {
        let filter_dir = AddToGithub::new::<&str>(&[]);
        let bogus_dir = Path::new("/Users/no_such_user/");
        assert_eq!(filter_dir.filter(&bogus_dir), false);
    }
}
