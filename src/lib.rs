//! Scanning directories in parallel.
//!
//! # Examples
//!
//! ```
//! use std::path::Path;
//!
//! let filter = |_: &Path| true;
//! let map = |path: &Path, _| Ok(path.exists());
//! let (paths, results): (Vec<_>, Vec<_>) = folder::scan("src", filter, map, (), None).unzip();
//! ```

use std::io::Result;
use std::ops::Deref;
use std::path::PathBuf;

use walkdir::WalkDir;

/// Process a path in parallel.
///
/// The function traverses files in a given path, selects those satisfying a criterion, and
/// processes the chosen ones in parallel, returning the corresponding results.
///
/// # Arguments
///
/// * `path` is the location to scan;
/// * `filter` is a function for choosing files, which is be invoked sequentially;
/// * `map` is a function for processing files, which is be invoked in parallel;
/// * `context` is an context passed to the processing function; and
/// * `workers` is the number of threads to use.
pub fn scan<Path, Filter, Map, Context, Value>(
    path: Path,
    filter: Filter,
    map: Map,
    context: Context,
    workers: Option<usize>,
) -> impl DoubleEndedIterator<Item = (PathBuf, Result<Value>)>
where
    Path: AsRef<std::path::Path>,
    Filter: Fn(&std::path::Path) -> bool + Copy,
    Map: Fn(&std::path::Path, Context) -> Result<Value> + Copy + Send + 'static,
    Context: Clone + Send + 'static,
    Value: Send + 'static,
{
    r#loop::parallelize(
        WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| !entry.file_type().is_dir())
            .filter(move |entry| filter(entry.path()))
            .map(|entry| entry.path().to_owned()),
        move |path, context| map(path.deref(), context),
        context,
        workers,
    )
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn nonexistent() {
        let _: Vec<_> =
            super::scan(Path::new("foo"), |_| true, |_, _| Ok(true), (), None).collect();
    }
}
