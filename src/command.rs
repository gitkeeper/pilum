//! # Commands
//!
//! This module contains functions that implement the command-line interface for
//! the application.
//!
use crate::{task::Task, utilities::*, Result};
use surrealdb::{engine::local::Db, Surreal};

/// Adds a new pending task to the task list.
///
/// # Parameters
///
/// - `db`: A reference to the SurrealDB database.
/// - `name`: The name of the task to be added.
///
/// # Panics
/// The function will panic if the database connection fails.
///
/// # Errors
/// The function will return an error if the database connection fails.
///
pub async fn add_tasks(db: &Surreal<Db>, names: Vec<String>) -> Result<()> {
    for name in names {
        let number = next_task_number(db).await?;
        let task = Task::new(number, name);
        let created = db.create(Task::TABLE).content(task).await?;
        let task = created.first().ok_or("Failed to add task.")?;
        print_task_action("Created", task);
    }

    Ok(())
}

/// Completes the specified tasks in the task list.
///
/// # Parameters
///
/// - `db`: A reference to the SurrealDB database.
/// - `numbers`: A list of task numbers to be completed.
///
/// # Returns
///
/// The function returns `Ok(())` if the operation is successful.
///
/// # Errors
///
/// The function will return an error if the database query fails.
///
pub async fn complete_tasks(db: &Surreal<Db>, numbers: Vec<i64>) -> Result<()> {
    let tasks = select_tasks(db, numbers).await?;

    let mut counter = 0;
    for mut task in tasks {
        task.complete().update(db).await?;
        print_task_action("Completed", &task);
        counter += 1;
    }

    print_task_action_summary("Completed", counter);
    Ok(())
}

/// Lists all the tasks in the task list.
///
/// # Parameters
///
/// - `db`: A reference to the SurrealDB database.
///
/// # Returns
///
/// The function returns `Ok(())` if the operation is successful.
///
/// # Errors
///
/// The function will return an error if the database query fails.
///
pub async fn list_all_tasks(db: &Surreal<Db>) -> Result<()> {
    let sql = "SELECT * FROM tasks ORDER BY number ASC";
    let tasks: Vec<Task> = db.query(sql).await?.take(0)?;

    print_task_list(&tasks);
    Ok(())
}

/// Lists all completed tasks from the database.
///
/// # Parameters
///
/// - `db`: A reference to the SurrealDB database.
///
/// # Returns
///
/// Returns a `Result` indicating success or failure.
///
/// # Errors
///
/// The function will return an error if the database query fails.
///
pub async fn list_completed_tasks(db: &Surreal<Db>) -> Result<()> {
    let sql = "SELECT * FROM tasks WHERE status = 'Completed' ORDER BY number ASC";
    let tasks: Vec<Task> = db.query(sql).await?.take(0)?;

    print_task_list(&tasks);
    Ok(())
}

/// Gets the next task number.
///
/// The function takes a reference to the SurrealDB database and returns the
/// next task number to be used. It queries the database for the existing tasks,
/// finds the maximum task number, and increments it by one to get the next task
/// number.
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
    let tasks: Vec<Task> = db.select(Task::TABLE).await?;
    let next_number = tasks.iter().map(|t| t.number()).max().unwrap_or(0) + 1;

    Ok(next_number)
}

/// Selects and returns the tasks with the specified task numbers from the
/// database in ascending order.
///
/// # Parameters
///
/// - `db`: A reference to the SurrealDB database.
/// - `numbers`: A vector of task numbers to be selected.
///
/// # Returns
///
/// A vector of tasks with the specified task numbers from the database, if
/// present, ordered in ascending order by task number.
///
/// # Errors
///
/// Returns an error if the database query fails.
///
async fn select_tasks(db: &Surreal<Db>, numbers: Vec<i64>) -> Result<Vec<Task>> {
    let sql = format!(
        "SELECT * FROM tasks WHERE number INSIDE {} ORDER BY number ASC",
        vec_to_array(&numbers)
    );
    let tasks: Vec<Task> = db.query(sql).await?.take(0)?;

    Ok(tasks)
}
