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
pub mod cli;
pub mod database;

use crate::database::Database;

use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Db;
use surrealdb::Surreal;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Task {
    number: i64,
    name: String,
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
pub async fn add_task(names: Vec<String>) -> Result<()> {
    let db = Database::initialize().await?;

    for name in names {
        let number = next_task_number(&db).await?;
        let created: Vec<Task> = db.create("tasks").content(Task { number, name }).await?;
        let task = created.first().unwrap();

        println!("Created task {}: {}", task.number, task.name);
    }

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
async fn next_task_number(db: &Surreal<Db>) -> Result<i64> {
    let tasks: Vec<Task> = db.select("tasks").await?;
    let next_number = tasks.iter().map(|t| t.number).max().unwrap_or(0) + 1;
    Ok(next_number)
}

/// Lists all the tasks in the task list.
///
/// The `list_all_tasks` function initializes the SurrealDB database and queries
/// the database for all the tasks. It then prints the task number and name for
/// each task found in the database.
///
/// # Parameters
///
/// There are no parameters for this function.
///
/// # Returns
///
/// The function returns `Ok(())` if the operation is successful.
///
/// # Errors
///
/// The function will return an error if the database query fails.
///
async fn list_all_tasks() -> Result<()> {
    let db = Database::initialize().await?;
    let mut response = db.query("SELECT * FROM tasks ORDER BY number ASC").await?;
    let tasks: Vec<Task> = response.take(0)?;

    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        for task in tasks {
            println!("{}: {}", task.number, task.name);
        }
    }

    Ok(())
}
