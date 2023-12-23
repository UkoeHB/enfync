# Changelog

## [0.1.4]

### Fixed

- Do not require `Clone + Default` on `Handle`. This fixes the inability to box a `Handle` caused by the `Sized` bound.


## [0.1.3]

### Fixed

- Do not require `TryAdopt` on `Handle`. This fixes the inability to box a `Handle` caused by the `Sized` bound on `TryAdopt`.
- Switch from `glootimer` to `wasmtimer` for `sleep()` since glootimer `TimeoutFuture` is `!Send`. WASM tests no longer work with `wasm-pack test --node` due to an obscure error caused by `wasmtimer`, so we need to test with a browser instead.


## [0.1.2]

### Fixed

- `SimpleResultReceiver::try_extract()` now works properly.


## [0.1.1]

### Changed

- Ignore tokio default features.


## [0.1.0]

### Added

- Initial release.
