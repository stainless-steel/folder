//! Scanning directories in parallel.

use std::io::Result;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

use walkdir::WalkDir;

/// Process an iterator in parallel.
pub fn parallelize<I, M, E, C, V>(
    iterator: I,
    map: M,
    context: C,
    workers: usize,
) -> impl Iterator<Item = (E, Result<V>)> + DoubleEndedIterator
where
    I: Iterator<Item = E>,
    M: Fn(&E, C) -> Result<V> + Copy + Send + 'static,
    E: Send + 'static,
    C: Clone + Send + 'static,
    V: Send + 'static,
{
    let (forward_sender, forward_receiver) = mpsc::channel::<E>();
    let (backward_sender, backward_receiver) = mpsc::channel::<(E, Result<V>)>();
    let forward_receiver = Arc::new(Mutex::new(forward_receiver));

    let _: Vec<_> = (0..workers)
        .map(|_| {
            let forward_receiver = forward_receiver.clone();
            let backward_sender = backward_sender.clone();
            let context = context.clone();
            thread::spawn(move || loop {
                let entry = match forward_receiver.lock().unwrap().recv() {
                    Ok(entry) => entry,
                    Err(_) => break,
                };
                let result = map(&entry, context.clone());
                backward_sender.send((entry, result)).unwrap();
            })
        })
        .collect();
    let mut count = 0;
    for entry in iterator {
        forward_sender.send(entry).unwrap();
        count += 1;
    }
    (0..count).map(move |_| backward_receiver.recv().unwrap())
}

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
///
/// # Examples
///
/// ```
/// let results: Vec<_> = folder::scan(
///     "src",
///     |path| true,
///     |path, _| Ok(path.exists()),
///     (),
///     1,
/// )
/// .collect();
/// assert_eq!(format!("{results:?}"), r#"[("src/lib.rs", Ok(true))]"#);
/// ```
pub fn scan<P, F, M, C, V>(
    path: P,
    filter: F,
    map: M,
    context: C,
    workers: usize,
) -> impl Iterator<Item = (PathBuf, Result<V>)> + DoubleEndedIterator
where
    P: AsRef<Path>,
    F: Fn(&Path) -> bool + Copy,
    M: Fn(&Path, C) -> Result<V> + Copy + Send + 'static,
    C: Clone + Send + 'static,
    V: Send + 'static,
{
    parallelize(
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
        let _: Vec<_> = super::scan(Path::new("foo"), |_| true, |_, _| Ok(true), (), 1).collect();
    }
}
