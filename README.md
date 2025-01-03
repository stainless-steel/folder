# Folder [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package allows for scanning directories in parallel.

## Examples

Synchronously:

```rust
use std::path::{Path, PathBuf};

use folder::scan;

let filter = |path: &Path| path.ends_with(".rs");
let map = |path: PathBuf, _| path.metadata().unwrap().len();
let fold = |sum, value| sum + value;
let _ = scan("src", filter, map, (), None).fold(0, fold);
```

Asynchronously:

```rust
use std::path::{Path, PathBuf};

use folder::asynchronous::scan;
use futures::stream::StreamExt;

let filter = |path: &Path| path.ends_with(".rs");
let map = |path: PathBuf, _| async move { path.metadata().unwrap().len() };
let fold = |sum, value| async move { sum + value };
let _ = scan("src", filter, map, (), None).fold(0, fold).await;
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[build-img]: https://github.com/stainless-steel/folder/workflows/build/badge.svg
[build-url]: https://github.com/stainless-steel/folder/actions/workflows/build.yml
[documentation-img]: https://docs.rs/folder/badge.svg
[documentation-url]: https://docs.rs/folder
[package-img]: https://img.shields.io/crates/v/folder.svg
[package-url]: https://crates.io/crates/folder
