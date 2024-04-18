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
pub mod error;

use crate::error::Error;

use database::Database;
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Db;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
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
    let tasks: Vec<Task> = db.select("tasks").await?;
    let next_number = tasks.iter().map(|t| t.number).max().unwrap_or(0) + 1;
    Ok(next_number)
}
