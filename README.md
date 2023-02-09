# Folder [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package allows for scanning directories in parallel.

## Example

```rust
use std::path::Path;

let results: Vec<_> = folder::scan(
    Path::new("src"),
    |path| true,
    |path, _| Ok(path.exists()),
    (),
    1,
)
.collect();
assert_eq!(format!("{results:?}"), r#"[("src/lib.rs", Ok(true))]"#);
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
