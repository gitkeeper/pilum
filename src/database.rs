//! # Database
//!
//! This module provides the `Database` struct which is used to interact with
//! the SurrealDB database. This struct provides methods for initializing and
//! interacting with the database.
//!
//! The `initialize` method is used to create a new instance of the `Database`
//! struct and connect to //! the SurrealDB database. It uses the `NAMESPACE`
//! and `DATABASE` constants to specify the namespace and database for SurrealDB
//! to use.
//!
//! The `path` method is used to determine the path to the database file. It
//! uses the `dirs` crate to get the home directory and appends the default
//! directory to it.
//!
use crate::Result;

use std::path::PathBuf;
use surrealdb::{
    engine::local::{Db, RocksDb},
    Surreal,
};

/// The `Database` struct provides a way to interact with the SurrealDB database.
/// It does not hold any data itself, but provides methods for initializing and
/// interacting with the database.
///
#[derive(Debug)]
pub struct Database;

impl Database {
    pub const NAMESPACE: &'static str = "pilum";
    pub const DATABASE: &'static str = "database";

    /// Initializes a new instance of the `Database` struct and connects to the
    /// SurrealDB production database. It uses the `NAMESPACE` and `DATABASE`
    /// constant to specify the namespace and database for SurrealDB to use.
    ///
    /// The function returns a `Result` that contains the `Surreal<Db>` instance if
    /// the database connection is successful, or an `Error` if the connection
    /// fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the SurrealDB database connection
    /// fails. This could be due to a variety of reasons, such as the database file
    /// not existing, database lock or a network error if the database is remote.
    ///
    /// # Notes
    ///
    /// In normal mode, the database resides is the user's home directory inside a
    /// hidden directory called `.pilum`. In test mode, the database resides in a
    /// temporary directory. This is to ensure that the test database is isolated
    /// from the production database.
    ///
    /// Use `Database::cleanup_test()` to remove the test database directory after
    /// the tests have been run. Alternatively, you can also remove the production
    /// database directory using `Database::cleanup()`.
    ///
    pub async fn initialize() -> Result<Surreal<Db>> {
        let mut endpoint = Self::namespace_production()?.join(Self::DATABASE);

        if std::env::var("PILUM_MODE").is_ok_and(|m| m == "test") {
            endpoint = Self::namespace_test()?.join(Self::DATABASE);
        }

        Self::connect(endpoint).await
    }

    /// Connects to the SurrealDB database at the specified endpoint.
    ///
    /// The endpoint is the path to the database file. The function returns a
    /// `Result` that contains the `Surreal<Db>` instance if the connection is
    /// successful, or an `Error` if the connection fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the SurrealDB database connection
    /// fails. This could be due to a variety of reasons, such as the database file
    /// not existing, database lock or a network error if the database is remote.
    ///
    async fn connect(endpoint: PathBuf) -> Result<Surreal<Db>> {
        let db = Surreal::new::<RocksDb>(endpoint).await?;
        db.use_ns(Self::NAMESPACE).use_db(Self::DATABASE).await?;
        Ok(db)
    }

    /// Removes the production database directory.
    ///
    /// This method is used to remove the production database directory. This
    /// function deletes the whole application directory in production!
    ///
    /// # Returns
    ///
    /// The function returns `Ok(())` if the operation is successful.
    ///
    /// # Errors
    ///
    /// The function will return an error if the directory's removal fails  or
    /// doesn't exist.
    ///
    pub fn cleanup_production() -> Result<()> {
        Self::cleanup(Self::namespace_production()?)
    }

    /// Removes the test database directory.
    ///
    /// This method is used to remove the test database directory. It is typically
    /// used after running the tests to clean up the test database.
    ///
    /// # Returns
    ///
    /// The function returns `Ok(())` if the operation is successful.
    ///
    /// # Errors
    ///
    /// The function will return an error if the directory's removal fails  or
    /// doesn't exist.
    ///
    pub fn cleanup_test() -> Result<()> {
        Self::cleanup(Self::namespace_test()?)
    }

    /// Removes the directory at the given path if it exists.
    ///
    /// # Parameters
    ///
    /// - `path`: the path to the directory that should be removed.
    ///
    /// # Returns
    ///
    /// The function returns `Ok(())` if the operation is successful.
    ///
    /// # Errors
    ///
    /// The function will return an error if the directory's removal fails  or
    /// doesn't exist.
    ///
    fn cleanup(path: PathBuf) -> Result<()> {
        if path.exists() {
            std::fs::remove_dir_all(&path)?
        }
        Ok(())
    }

    /// Returns the path where the production database resides in.
    ///
    /// # Returns
    ///
    /// This function returns the path where the production database resides in.
    ///
    fn namespace_production() -> Result<PathBuf> {
        let home_dir = dirs::home_dir().ok_or("Failed to get home directory.")?;
        Ok(home_dir.join(format!(".{}", Self::NAMESPACE)))
    }

    /// Returns the path where the test database resides in.
    ///
    /// # Returns
    ///
    /// This function returns the path where the test database resides in.
    ///
    fn namespace_test() -> Result<PathBuf> {
        let current_dir = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;
        Ok(current_dir.join("tmp").join(Self::NAMESPACE))
    }
}
