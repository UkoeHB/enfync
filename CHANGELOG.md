# Changelog

## [0.1.3]

### Fixed

- Do not require `TryAdopt` on `Handle`. This fixes the inability to box a `Handle` caused by the `Sized` bound on `TryAdopt`.


## [0.1.2]

### Fixed

- `SimpleResultReceiver::try_extract()` now works properly.


## [0.1.1]

### Changed

- Ignore tokio default features.


## [0.1.0]

### Added

- Initial release.
