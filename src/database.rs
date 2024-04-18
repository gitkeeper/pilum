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
use crate::Error;

use std::path::PathBuf;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;

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
    pub async fn initialize() -> Result<Surreal<Db>, Error> {
        let mut endpoint = Self::namespace_production()?.join(Self::DATABASE);

        if std::env::var("PILUM_MODE").is_ok_and(|m| m == "test") {
            endpoint = Self::namespace_test().join(Self::DATABASE);
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
    async fn connect(endpoint: PathBuf) -> Result<Surreal<Db>, Error> {
        let db = Surreal::new::<RocksDb>(endpoint).await?;
        db.use_ns(Self::NAMESPACE).use_db(Self::DATABASE).await?;
        Ok(db)
    }

    /// Removes the production database directory.
    ///
    /// This method is used to remove the database directory. This function
    /// deletes the whole application directory in production!
    ///
    pub fn cleanup_production() -> std::io::Result<()> {
        std::fs::remove_dir_all(Self::namespace_production().unwrap())
    }

    /// Removes the test database directory.
    ///
    /// This method is used to remove the test database directory. It is typically
    /// used after running the tests to clean up the test database.
    ///
    pub fn cleanup_test() -> std::io::Result<()> {
        std::fs::remove_dir_all(Self::namespace_test())
    }

    /// Returns the path where the production database resides in.
    fn namespace_production() -> Result<PathBuf, Error> {
        let namespace = dirs::home_dir()
            .ok_or(Error::HomeDirNotFound)?
            .join(format!(".{}", Self::NAMESPACE));
        Ok(namespace)
    }

    /// Returns the path where the test database resides in.
    fn namespace_test() -> PathBuf {
        std::env::temp_dir().join(Self::NAMESPACE)
    }
}
