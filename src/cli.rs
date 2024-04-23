//! The `cli` module contains the command-line interface for the application.
//!
//! The `Cli` struct is responsible for parsing command-line arguments and
//! executing the corresponding commands. It uses the `Commands` enum to define
//! the available commands for the application.
//!
use crate::database::Database;
use crate::{command::*, Result};
use clap::{Parser, Subcommand};

/// The `Commands` enum defines the available commands for the application.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Shows active tasks.
    Active,
    /// Adds a new pending task to the task list.
    Add {
        /// The name or multiple names for tasks to add.
        names: Vec<String>,
    },
    /// Shows all tasks.
    All,
    /// Shows completed tasks.
    Completed,
    /// Deletes the specified task.
    Delete,
    /// Marks the specified task as completed.
    Done {
        /// The task number to mark as done.
        numbers: Vec<i64>,
    },
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
    pub async fn run() -> Result<()> {
        let args = Cli::parse();
        let db = Database::initialize().await?;

        match args.command {
            Some(Commands::Add { names }) => add_task(&db, names).await?,
            Some(Commands::All) => list_all_tasks(&db).await?,
            Some(Commands::Done { numbers }) => complete_task(&db, numbers).await?,
            None => exit_no_subcommand(),
            _ => exit_unknown_subcommand(),
        }

        Ok(())
    }
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
    /// ensure the asserts are evaluated, this will run those assertions in a way
    /// convenient for running as a test.
    ///
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
