//! Scanning directories in parallel.
//!
//! # Examples
//!
//! Synchronously:
//!
//! ```
//! fn main() {
//!     use std::path::{Path, PathBuf};
//!
//!     use folder::scan;
//!
//!     let filter = |path: &Path| path.ends_with(".rs");
//!     let map = |path: PathBuf, _| path.metadata().unwrap().len();
//!     let fold = |sum, value| sum + value;
//!     let _ = scan("src", filter, map, (), None).fold(0, fold);
//! }
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
//!     use folder::asynchronous::scan;
//!
//!     let filter = |path: &Path| path.ends_with(".rs");
//!     let map = |path: PathBuf, _| async move { path.metadata().unwrap().len() };
//!     let fold = |sum, value| async move { sum + value };
//!     let _ = scan("src", filter, map, (), None).fold(0, fold).await;
//! }
//! # #[cfg(not(feature = "asynchronous"))]
//! # fn main() {}
//! ```

#[cfg(feature = "asynchronous")]
pub mod asynchronous;
pub mod synchronous;

pub use synchronous::scan;
