//! # Utilities
//!
//! This module contains common utility functions that are shared across the
//! whole application.
//!
use crate::task::Task;
use std::fmt::Display;

/// Converts a vector of items to a string array.
///
/// # Parameters
///
/// - `items`: A reference to a vector of items.
///
/// # Returns
///
/// The function returns a string representation of the items in an array.
///
pub fn vec_to_array<T: Display>(items: &[T]) -> String {
    let items = items
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    format!("[{}]", items)
}

/// Prints the number and name of each task in the given vector as a list.
///
/// # Parameters
///
/// - `tasks` - A vector containing tasks.
///
pub fn print_task_list(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        for task in tasks {
            task.print_number_and_name();
        }
    }
}

/// Prints an action message for the specified task.
///
/// # Parameters
///
/// - `action`: The action performed on the tasks.
/// - `counter`: The number of tasks the action was applied to.
///
pub fn print_task_action(action: &str, task: &Task) {
    println!("{} task {} '{}'.", action, task.number(), task.name());
}

/// Prints a summary message for the specified task action.
///
/// # Parameters
///
/// - `action`: The action performed on the tasks.
/// - `counter`: The number of tasks the action was applied to.
///
pub fn print_task_action_summary(action: &str, counter: i64) {
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
