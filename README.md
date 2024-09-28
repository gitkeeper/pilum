# Pilum

[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE.md)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT.md)
[![Build Status](https://img.shields.io/github/actions/workflow/status/gitkeeper/pilum/rust.yml?branch=main&style=flat-square)](https://github.com/gitkeeper/pilum/actions/workflows/rust.yml?query=branch%3Amain)
[![Coverage Status](https://img.shields.io/coveralls/github/gitkeeper/pilum/main?style=flat-square)](https://coveralls.io/github/gitkeeper/pilum?branch=main)

## About

Pilum is a simple task manager with a CLI written in Rust.

Pilum serves as a convenient and easy-to-use task management tool, operated via a command line
interface. It keeps track of your to-dos with commands for adding, changing and removing tasks.

## Installation

For the time being, you must have [Rust](https://www.rust-lang.org/) installed on your machine.
To install Rust, it's recommended to follow
[Rust's installation instructions](https://www.rust-lang.org/tools/install) for your respective
operating system.

Afterward, you can install Pilum by running `cargo install pilum`.

## Usage

Pilum's command-line interface is invoked with the following pattern:

`pilum`

… and then by entering a number corresponding the action to take.

For further information run `pilum --help`.

## Development

After checking out the repo, run `cargo check` followed by `cargo build` to install dependencies.
Then, run`cargo test` to run the tests. You can also run `cargo run` to invoke the task manager
and add some arguments and/or options to play around with the commands available.

If you work on new features, you should also add the proper documentation. Please run `cargo doc`
before you create a pull request and especially before you publish. To release a new version,
update the version number in [Cargo.toml](Cargo.toml), and then run `cargo publish`, which will
upload the package to [crates.io](https://crates.io). The project adheres to the
[semantic versioning](https://semver.org/) standard.

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/gitkeeper/pilum. This
project is intended to be a safe, welcoming space for collaboration, and contributors are expected
to adhere to the [code of conduct](CODE_OF_CONDUCT.md).

## License

This package is available as open source and dual-licensed under the terms of
[Apache 2.0](LICENSE-APACHE.md) and [MIT](LICENSE-MIT.md). The user or developer choosing to use,
distribute, or modify this software can opt to do so under either of these licenses, adhering to
the terms and conditions defined by the license they choose.
