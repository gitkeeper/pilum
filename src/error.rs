//! Error type for the application.
//!
//! This module defines the `Error` type for the application. The `Error` type
//! is used to represent all the possible errors that can occur during the
//! execution of the application. It is an enum that contains variants for
//! different types of errors, such as database errors, file system errors, etc.
//!
//! The `Error` type is used throughout the application to propagate errors
//! from one part of the application to another. It is also used to provide
//! more context about the error that occurred, such as the reason for the error
//! and the source of the error.
//!

/// Error types for the application.
#[derive(Debug)]
pub enum Error {
    DatabaseError(surrealdb::Error),
    HomeDirNotFound,
    TempDirCreationFailed,
}

impl From<surrealdb::Error> for Error {
    /// Converts a `surrealdb::Error` into an `Error`.
    fn from(error: surrealdb::Error) -> Error {
        Error::DatabaseError(error)
    }
}
