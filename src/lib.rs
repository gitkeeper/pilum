use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Shows active tasks.
    Active,
    /// Adds a new pending task to the task list.
    Add,
    /// Shows all tasks.
    All,
    /// Appends text to an existing task description.
    Append,
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
    /// Prepends text to an existing task description.
    Prepend,
}

/// Pilum is a sophisticated task manager with a CLI and a GUI written in Rust.
///
/// Pilum serves as a convenient and easy-to-use task management tool, operated via the command line
/// and a graphical interface. It keeps track of your to-do tasks, enabling operations like adding,
/// removing and altering tasks as per your requirements. Pilum is equipped with a wide range of
/// commands for sophisticated task manipulations.
///
/// Essentially, Pilum functions as a list organizer. You can feed details, along with their
/// respective parameters, and the program neatly structures and displays it. By integrating
/// deadlines and recurring tasks, it becomes a comprehensive to-do manager. Further refinement
/// is achieved by incorporating elements like priorities, tags, project groups and more, making
/// Pilum a fully-fledged task organization program.
#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub fn run() {
        let cli = Cli::parse();

        if let Some(command) = &cli.command {
            println!("Command: {:?}", command)
        }
    }
}

/// `clap` reports most development errors as `debug_assert!`s. Rather than checking every
/// subcommand, this test catches possible problems earlier in the development cycle.
///
/// Most error states are handled as asserts under the assumption they are programming mistake and
/// not something to handle at runtime. Rather than relying on tests (manual or automated) that
/// exhaustively test the CLI to ensure the asserts are evaluated, this will run those asserts in a
/// way convenient for running as a test.
#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
