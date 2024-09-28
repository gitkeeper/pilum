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

- None

### Fixed

- None

### Security

- None

## [0.2.0] - 2024-09-28

### Added

- The main function loops through input numbers to execute a certain command.
- Added a `Task` struct to hold a task's data like its id, its title and if it's completed or not.
- Added a `TaskList` to hold all tasks inside a vector to iterate over.
- Implemented `new`, `add_task`, `list_tasks` and `complete_task` for `TaskList`.

## [0.1.0] - 2024-09-28

### Added

- Added an automated workflow to build and test the application on GitHub.
- Prepared a [CHANGELOG.md](CHANGELOG.md) that is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
- Wrote the starting [README.md](README.md) with some placeholder paragraphs for now.
- Included a [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) as a guideline for collaboration on this project.
- Added the [Apache 2.0](LICENSE-APACHE.md) and the [MIT](LICENSE-MIT.md) licenses for this project.
- Set up [Cargo.toml](Cargo.toml) with initial metadata for this package.
- Configured [.gitignore](.gitignore) to ignore Rust related directories and temporary files.
- Founded a new binary package with [Cargo](https://doc.rust-lang.org/cargo/).
