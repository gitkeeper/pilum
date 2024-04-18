//! # Library
//!
//! This is the core library for the application.
//!
//! The `lib.rs` file contains the main structures and logic of the application.
//! It includes the `Cli` struct which is responsible for parsing command-line
//! arguments and executing the corresponding commands. It also includes the
//! `Commands` enum which defines the available commands for the application as
//! well as their actual implementations.
//!
//! The `Database` module is imported in this file. It uses the SurrealDB
//! database for data persistence. The `Database` struct is used to initialize
//! and interact with the SurrealDB database. It must be used inside an
//! asynchronous functions and methods due to its asynchronous nature.
//!
pub mod database;

use clap::{Parser, Subcommand};
use database::Database;
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Db;
use surrealdb::Surreal;

#[derive(Debug)]
pub enum Error {
    DatabaseError(surrealdb::Error),
    HomeDirNotFound,
    TempDirCreationFailed,
}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Error {
        Error::DatabaseError(error)
    }
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    number: i64,
    name: String,
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
    /// If the database initialization is successful, it proceeds to check if a
    /// command was provided through the command-line arguments. If a command is
    /// found, it executes the command.
    ///
    /// This function is asynchronous because the commands themselves are
    /// asynchronous.
    ///
    /// # Errors
    ///
    /// The function will return an error if the database connection fails.
    ///
    pub async fn run() -> Result<(), Error> {
        let args = Cli::parse();

        match args.command {
            Some(Commands::Add { name }) => add_task(name).await?,
            None => exit_no_subcommand(),
            _ => exit_unknown_subcommand(),
        }

        Ok(())
    }
}

/// Adds a new pending task to the task list.
///
/// The `add_task` function takes a `name` parameter, which is the name of the task
/// to be added. It then initializes the SurrealDB database and prints a message
/// indicating that the task has been created.
///
/// The function is asynchronous because it calls the `Database::new` method, which
/// is asynchronous.
///
/// # Parameters
/// - `name`: The name of the task to be added.
///
/// # Panics
/// The function will panic if the database connection fails.
///
/// # Errors
/// The function will return an error if the database connection fails.
///
pub async fn add_task(name: String) -> Result<(), Error> {
    let db = Database::initialize().await?;

    let number = next_task_number(&db).await?;
    let created: Vec<Task> = db.create("tasks").content(Task { number, name }).await?;
    let task = created.first().unwrap();

    println!("Created task {}: {}", task.number, task.name);

    Ok(())
}

/// Gets the next task number.
///
/// The `next_task_number` function takes a reference to the SurrealDB database
/// and returns the next task number to be used. It queries the database for the
/// existing tasks, finds the maximum task number, and increments it by one to
/// get the next task number.
///
/// # Parameters
///
/// - `db`: A reference to the SurrealDB database.
///
/// # Returns
///
/// The function returns the next task number to be used.
///
/// # Errors
///
/// The function will return an error if the database query fails.
///
async fn next_task_number(db: &Surreal<Db>) -> Result<i64, Error> {
    let selected: Vec<Task> = db.select("tasks").await?;
    let next_number = selected.iter().map(|t| t.number).max().unwrap_or(0) + 1;
    Ok(next_number)
}

/// Exits the program with an error message if no subcommand is provided.
/// The program exits with a status code of 2 according to `clap`s behaviour.
fn exit_no_subcommand() {
    eprintln!("error: no subcommand specified\n\nUsage: pilum [COMMAND]\n\nFor more information, try '--help'.");
    std::process::exit(2);
}

/// Exits the program with an error message if an unknown subcommand is provided.
/// The program exits with a status code of 2 according to `clap`s behaviour.
fn exit_unknown_subcommand() {
    eprintln!("error: subcommand not implemented\n\nUsage: pilum [COMMAND]\n\nFor more information, try '--help'.");
    std::process::exit(2);
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
