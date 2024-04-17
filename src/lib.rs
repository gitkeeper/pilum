//! # Library
//!
//! This is the core library for the application.
//!
//! The `lib.rs` file contains the main structures and logic of the application.
//! It includes the `Cli` struct which is responsible for parsing command-line
//! arguments and executing the corresponding commands. It also includes the
//! `Commands` enum which defines the available commands for the application.
//!
//! The `Database` module is also imported in this file. It uses the SurrealDB
//! database for data persistence. The `Database` struct is used to initialize
//! and interact with the SurrealDB database. It must be used inside an
//! asynchronous context due to its asynchronous nature.
//!
pub mod database;

use clap::{Parser, Subcommand};
use database::Database;
use std::process::exit;

/// The `Commands` enum defines the available commands for the application.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Shows active tasks.
    Active,
    /// Adds a new pending task to the task list.
    Add {
        /// The name of the task.
        name: String,
    },
    /// Shows all tasks.
    All,
    /// Shows completed tasks.
    Completed,
    /// Deletes the specified task.
    Delete,
    /// Marks the specified task as completed.
    Done,
    /// Duplicates the specified tasks.
    Duplicate,
    /// Shows most details of tasks.
    List,
    /// Modifies the existing task with provided arguments.
    Modify,
}

/// Pilum is a sophisticated task manager with a CLI and a GUI written in Rust.
///
/// Pilum serves as a convenient and easy-to-use task management tool, operated
/// via the command line and a graphical interface. It keeps track of your to-do
/// tasks, enabling operations like adding, removing and altering tasks as per
/// your requirements. Pilum is equipped with a wide range of commands for
/// sophisticated task manipulations.
///
/// Essentially, Pilum functions as a list organizer. You can feed details,
/// along with their respective parameters, and the program neatly structures
/// and displays it. By integrating deadlines and recurring tasks, it becomes a
/// comprehensive to-do manager. Further refinement is achieved by incorporating
/// elements like priorities, tags, project groups and more, making Pilum a
/// fully-fledged task organization program.
///
#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    /// The `run` function is the main entry point for the application.
    ///
    /// It first parses the command-line arguments using the `Cli::parse` method.
    /// Then, it initializes the SurrealDB database by calling the
    /// `Database::initialize` method.
    ///
    /// If the database initialization is successful, it proceeds to check if a
    /// command was provided through the command-line arguments. If a command is
    /// found, it executes the command.
    ///
    /// This function is asynchronous because the `Database::initialize` method is
    /// asynchronous.
    ///
    pub async fn run() {
        let args = Cli::parse();

        match args.command {
            Some(Commands::Add { name }) => add_task(name).await,
            None => {
                eprintln!("error: no subcommand specified\n\nUsage: pilum [COMMAND]\n\nFor more information, try '--help'.");
                exit(2);
            }
            _ => {
                eprintln!("error: subcommand not implemented\n\nUsage: pilum [COMMAND]\n\nFor more information, try '--help'.");
                exit(2);
            }
        }
    }
}

async fn add_task(name: String) {
    let _db = Database::new().await;

    println!("Created task 1: {name}");

    // TODO: Implement the `add_task` function.
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `clap` reports most development errors as `debug_assert!`s. Rather than
    /// checking every subcommand, this test catches possible problems earlier in
    /// the development cycle.
    ///
    /// Most error states are handled as asserts under the assumption they are
    /// programming mistake and not something to handle at runtime. Rather than
    /// relying on tests (manual or automated) that exhaustively test the CLI to
    /// ensure the asserts are evaluated, this will run those asserts in a way
    /// convenient for running as a test.
    ///
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
