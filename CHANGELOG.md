# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- None

### Changed

- None

### Deprecated

- None

### Removed

- Removed the error module for now and simplified error handling in general by introducing a
  custom Result and Error type for the whole library.

### Fixed

- None

### Security

- None

## [0.2.1] - 2024-04-18

### Added

- Added the command `all` to display all tasks.

### Changed

- Updated the [README.md](README.md) to reflect the new command `all`.
- Improved module imports to be me logical: absolute paths (`use crate::`) for internal modules,
  external packages come after in their own block.
- Restructured the integration tests to work as scenarios that represent use cases.
- Misc integration tests now reside in their own test folder.

### Fixed

- Fixed the test runner so that it deploys the database inside a `tmp` directory in project root,
  which is also compatible for GitHub's actions.

### Removed

- Removed the setting `TMPDIR: ${{ runner.temp }}` in the [GitHub workflow](.github/workflows/rust.yml).

## [0.2.0] - 2024-04-18

### Added

- Added a library called `lib.rs` to extract the application's logic from `main.rs` to.
- Restructured the code base into modules: [cli.rs](src/cli.rs), [database.rs](src/database.rs)
  and `error.rs` (deprecated after [0.2.1](#021---2024-04-18)).
- Errors propagate from the Database through the modules up to `main.rs`.
- Giving an unknown or no subcommand at all throws an error similar to `clap`'s errors.
- Ensured that the environment variable `PILUM_MODE=test` puts the application in test mode.
- Introduced `trycmd` to enumerate test case files and run them to verify the command's results.
- Added test case files for the command `help`, `add` and `unknown` commands.
- `trycmd`'s TestCases function first deletes all test files and databases before running.

### Changed

- Elaborated comments for modules, associated functions and methods to give more information.

### Fixed

- Made some minor adjustments to the [README.md](README.md) to reflect the application's purpose.

## [0.1.0] - 2024-04-03

### Added

- Founded a new binary package with [Cargo](https://doc.rust-lang.org/cargo/).
- Created an initial folder structure with help of `.gitkeep`.
- Set up [Cargo.toml](Cargo.toml) with initial metadata for this package.
- Introduced [clap](https://github.com/clap-rs/clap) to be the command-line parser for the package.
- Wrote the starting [README.md](README.md) with some placeholder paragraphs for now.
- Prepared a [CHANGELOG.md](CHANGELOG.md) that is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
- Included a [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) as a guideline for collaboration on this project.
- Added the [Apache 2.0](LICENSE-APACHE.md) and the [MIT](LICENSE-MIT.md) licenses for this project.
