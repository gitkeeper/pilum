//! # Library
//!
//! This is the core library for the application.
//!
//! The `lib.rs` file links to all the modules in the library and re-exports them
//! for use in the application. The library contains the core functionality of the
//! application, such as the command-line interface, database operations, and task
//! management.
//!
//! The library is organized into modules, each representing a different aspect of
//! the application. The `cli` module contains the command-line interface logic,
//! the `command` module defines the available commands, the `database` module
//! handles database operations, and the `task` module defines the task structure
//! and operations.
//!
//! The library uses the `Result` and `Error` types to handle errors and return
//! results from functions. The `Result` type is a generic type that can hold a
//! successful value or an error value. The `Error` type is a trait object that
//! represents any type that implements the `Error` trait.
//!
pub mod cli;
pub mod command;
pub mod database;
pub mod task;

mod utilities;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;
