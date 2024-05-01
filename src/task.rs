//! # Task
//!
//! This module contains the `Task` struct with its implementations as well as
//! the `TaskStatus` enum with is various statuses.
//!
use crate::Result;
use serde::{Deserialize, Serialize};
use std::fmt;
use surrealdb::{engine::local::Db, sql::Thing, Surreal};

/// A task is a unit of work that needs to be done.
///
/// A task has a number and a name. The number is a unique identifier for the task
/// and the name is a short description of the task.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: Option<Thing>,
    number: i64,
    name: String,
    status: TaskStatus,
}

/// There are various statuses for a task.
///
/// A task can have exactly one status.
///
#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Completed,
    Started,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::Completed => write!(f, "Completed"),
            TaskStatus::Started => write!(f, "Started"),
        }
    }
}

impl Task {
    pub const TABLE: &'static str = "tasks";

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
            id: None,
            number,
            name,
            status: TaskStatus::Pending,
        }
    }

    /// Gets the task's number.
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

    /// Marks the task as completed and updates it in the database.
    ///
    /// # Parameters
    ///
    /// - `db` - A reference to a `Surreal<Db>` object representing the database.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the updated `Task` object if the operation is successful.
    ///
    /// # Errors
    ///
    /// Returns an error if there was a problem updating the task in the database.
    ///
    pub fn complete(&mut self) -> &Self {
        self.status = TaskStatus::Completed;
        self
    }

    /// Marks the task as started.
    ///
    /// # Parameters
    ///
    /// - `db` - A reference to a `Surreal<Db>` object representing the database.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the updated `Task` object if the operation is successful.
    ///
    /// # Errors
    ///
    /// Returns an error if there was a problem updating the task in the database.
    ///
    pub fn start(&mut self) -> &Self {
        self.status = TaskStatus::Started;
        self
    }

    /// Updates the task inside the database.
    ///
    /// # Parameters
    ///
    /// - `db` - A reference to a `Surreal<Db>` object representing the database.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the updated `Task` object if the operation is successful.
    ///
    /// # Errors
    ///
    /// Returns an error if there was a problem updating the task in the database.
    ///
    pub async fn update(&self, db: &Surreal<Db>) -> Result<Self> {
        let updated: Option<Self> = db.update(self.id()).content(self).await?;
        Ok(updated.unwrap())
    }

    /// Prints the task's number and name.
    ///
    pub fn number_and_name(&self) -> String {
        format!("{} '{}'", self.number(), self.name())
    }

    /// Gets the task's unique id given by the database.
    ///
    /// # Returns
    ///
    /// The function returns the task's unique id.
    ///
    fn id(&self) -> &Thing {
        self.id.as_ref().unwrap()
    }
}
