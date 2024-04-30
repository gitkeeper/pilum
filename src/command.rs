//! The `command` module contains functions that implement the command-line
//! interface for the application.
//!
//! Public functions:
//! - The public `add_task` function adds a new pending task to the task list.
//! - The public `list_all_tasks` function lists all the tasks in the task list.
//!
//! Private functions:
//! - The private `next_task_number` function gets the next task number to be used.
//!
use crate::{task::Task, Result};
use std::fmt::Display;
use surrealdb::{engine::local::Db, Surreal};

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
pub async fn add_task(db: &Surreal<Db>, names: Vec<String>) -> Result<()> {
    for name in names {
        let number = next_task_number(db).await?;
        let record: Vec<Task> = db.create("tasks").content(Task::new(number, name)).await?;
        let task = record.first().ok_or("Failed to add task.")?;
        print_task_action("Created", task);
    }

    Ok(())
}

/// Completes the specified tasks in the task list.
///
/// The `complete_task` function takes a list of task numbers and marks the tasks
/// with those numbers as completed. It initializes the SurrealDB database, queries
/// the database for the tasks with the specified numbers, and marks them as completed.
/// It then prints a message indicating that the tasks have been completed.
///
/// The function is asynchronous because it calls the `Database::query` method, which
/// is asynchronous.
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
pub async fn complete_task(db: &Surreal<Db>, numbers: Vec<i64>) -> Result<()> {
    let sql = format!(
        "SELECT * FROM tasks WHERE number INSIDE {} ORDER BY number ASC",
        vec_to_array(&numbers)
    );
    let tasks: Vec<Task> = db.query(sql).await?.take(0)?;

    let mut counter = 0;

    for mut task in tasks {
        task.complete(db).await?;
        print_task_action("Completed", &task);
        counter += 1;
    }

    print_task_action_summary("Completed", counter);

    Ok(())
}

/// Lists all the tasks in the task list.
///
/// The `list_all_tasks` function initializes the SurrealDB database and queries
/// the database for all the tasks. It then prints the task number and name for
/// each task found in the database.
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

    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        for task in tasks {
            println!("{} '{}'", task.number(), task.name());
        }
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
    let next_number = tasks.iter().map(|t| t.number()).max().unwrap_or(0) + 1;
    Ok(next_number)
}

/// Converts a vector of items to a string array.
///
/// The `vec_to_array` function takes a reference to a vector of items and
/// converts it to a string array. It maps each item to a string representation
/// and joins them with a comma separator.
///
/// # Parameters
///
/// - `items`: A reference to a vector of items.
///
/// # Returns
///
/// The function returns a string representation of the items in an array.
///
fn vec_to_array<T: Display>(items: &[T]) -> String {
    let items = items
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    format!("[{}]", items)
}

/// Prints a message for the specified task.
///
/// The `print_task_action` function takes an action and a task and prints a
/// message indicating the action performed on the task.
///
/// # Parameters
///
/// - `action`: The action performed on the tasks.
/// - `counter`: The number of tasks the action was applied to.
///
fn print_task_action(action: &str, task: &Task) {
    println!("{} task {} '{}'.", action, task.number(), task.name());
}

/// Prints a summary message for the specified task action.
///
/// The `print_task_action_summary` function takes an action and a counter and
/// prints a summary message indicating the number of tasks that given action
/// was applied to.
///
/// # Parameters
///
/// - `action`: The action performed on the tasks.
/// - `counter`: The number of tasks the action was applied to.
///
fn print_task_action_summary(action: &str, counter: i64) {
    match counter {
        1 => println!("{} {} task.", action, counter),
        _ => println!("{} {} tasks.", action, counter),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_vec_to_array_with_integers() {
        let vec: Vec<i64> = vec![1, 2, 3];
        let result = vec_to_array(&vec);
        assert_eq!(result, "[1, 2, 3]");
    }

    #[test]
    fn verify_vec_to_array_with_strings() {
        let vec: Vec<&str> = vec!["a", "b", "c"];
        let result = vec_to_array(&vec);
        assert_eq!(result, "[a, b, c]");
    }

    #[test]
    fn verify_vec_to_array_with_zero_items() {
        let vec: Vec<i64> = vec![];
        let result = vec_to_array(&vec);
        assert_eq!(result, "[]");
    }
}
