//! Scanning directories in parallel.
//!
//! # Examples
//!
//! Synchronously:
//!
//! ```
//! # #[cfg(not(feature = "asynchronous"))]
//! fn main() {
//!     use std::path::{Path, PathBuf};
//!
//!     let filter = |path: &Path| path.ends_with(".rs");
//!     let map = |path: PathBuf, _| path.metadata().unwrap().len();
//!     let _ = folder::scan("src", filter, map, ())
//!         .fold(0, |sum, value| sum + value);
//! }
//! # #[cfg(feature = "asynchronous")]
//! # fn main() {}
//!```
//!
//! Asynchronously:
//!
//!```
//! # #[cfg(feature = "asynchronous")]
//! #[tokio::main]
//! async fn main() {
//!     use std::path::{Path, PathBuf};
//!
//!     use futures::stream::StreamExt;
//!
//!     let filter = |path: &Path| path.ends_with(".rs");
//!     let map = |path: PathBuf, _| async move { path.metadata().unwrap().len() };
//!     let _ = folder::scan("src", filter, map, ())
//!         .fold(0, |sum, value| async move { sum + value })
//!         .await;
//! }
//! # #[cfg(not(feature = "asynchronous"))]
//! # fn main() {}
//! ```

#[cfg(feature = "asynchronous")]
#[path = "asynchronous.rs"]
mod implementation;

#[cfg(not(feature = "asynchronous"))]
#[path = "synchronous.rs"]
mod implementation;

/// Process a path in parallel.
///
/// The function traverses files in a given path, selects those satisfying a criterion, and
/// processes the chosen ones in parallel, returning the corresponding results.
///
/// # Arguments
///
/// * `path` is the location to scan;
/// * `filter` is a function for choosing files, which is be invoked sequentially;
/// * `map` is a function for processing files, which is be invoked in parallel; and
/// * `context` is an context passed to the processing function.
pub use implementation::scan;
