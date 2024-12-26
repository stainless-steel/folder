use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub fn scan<Root, Filter, Map, Context, Output>(
    root: Root,
    mut filter: Filter,
    mut map: Map,
    context: Context,
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
    )
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    #[test]
    fn scan() {
        let filter = |path: &Path| path.ends_with(".rs");
        let map = |path: PathBuf, _| path.metadata().unwrap().len();
        let _ = super::scan("src", filter, map, ()).fold(0, |sum, value| sum + value);
    }
}
