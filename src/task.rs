//! The `task` module contains the `Task` struct.
//!
//! The `Task` struct represents a task in the task list. It has a number and a name.
//! The number is a unique identifier for the task, and the name is a short description
//! of the task.
//!
//! The `Task` struct provides methods to create a new task, get the task number, and
//! get the task name.
//!
use serde::{Deserialize, Serialize};

/// A task is a unit of work that needs to be done.
///
/// A task has a number and a name. The number is a unique identifier for the task
/// and the name is a short description of the task.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    number: i64,
    name: String,
    status: TaskStatus,
}

#[derive(Debug, Serialize, Deserialize)]
enum TaskStatus {
    Pending,
    Completed,
}

impl Task {
    /// Creates a new task with the specified number and name.
    ///
    /// # Parameters
    ///
    /// - `number`: The number of the task.
    /// - `name`: The name of the task.
    ///
    /// # Returns
    ///
    /// The function returns a new `Task` instance with the specified number and name.
    ///
    pub fn new(number: i64, name: String) -> Self {
        Self {
            number,
            name,
            status: TaskStatus::Pending,
        }
    }

    /// Gets the number of the task.
    ///
    /// # Returns
    ///
    /// The function returns the number of the task.
    ///
    pub fn number(&self) -> i64 {
        self.number
    }

    /// Gets the name of the task.
    ///
    /// # Returns
    ///
    /// The function returns the name of the task.
    ///
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Marks the task as completed.
    ///
    /// The function changes the status of the task to `Completed`.
    ///
    /// # Returns
    ///
    /// The function returns a mutable reference to the task.
    ///
    pub fn complete(&mut self) -> &mut Self {
        self.status = TaskStatus::Completed;
        self
    }
}
