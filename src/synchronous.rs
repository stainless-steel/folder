//! Synchronous implementation.

use std::path::{Path, PathBuf};

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
/// * `workers` is the number of workers to use.
pub fn scan<Root, Filter, Map, Context, Output>(
    root: Root,
    mut filter: Filter,
    mut map: Map,
    context: Context,
    workers: Option<usize>,
) -> impl Iterator<Item = Output>
where
    Root: AsRef<Path>,
    Filter: FnMut(&Path) -> bool + Send + 'static,
    Map: FnMut(PathBuf, Context) -> Output + Copy + Send + 'static,
    Context: Clone + Send + 'static,
    Output: Send + 'static,
{
    let paths = WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| !entry.file_type().is_dir())
        .filter(move |entry| filter(entry.path()))
        .map(|entry| entry.path().to_owned());
    r#loop::parallelize(
        paths.zip(std::iter::repeat(context)),
        move |(path, context)| map(path, context),
        workers,
    )
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    #[test]
    fn scan() {
        let filter = |path: &Path| path.ends_with(".rs");
        let map = |path: PathBuf, _| path.metadata().unwrap().len();
        let fold = |sum, value| sum + value;
        let _ = super::scan("src", filter, map, (), None).fold(0, fold);
    }
}
