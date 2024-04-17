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
pub struct Database;

impl Database {
    const NAMESPACE: &'static str = "pilum";
    const DATABASE: &'static str = "database";

    /// Initializes a new instance of the `Database` struct and connects to the
    /// SurrealDB production database. It uses the `NAMESPACE` and `DATABASE`
    /// constant to specify the namespace and database for SurrealDB to use.
    /// In normal mode, the database resides is the user's home directory inside a
    /// hidden directory called `.pilum`.
    ///
    /// The function returns a `Result` that contains the `SurrealDb` instance if
    /// the database connection is successful, or a `SurrealError` if the connection
    /// fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the SurrealDB database connection
    /// fails. This could be due to a variety of reasons, such as the database file
    /// not existing, insufficient permissions or a network error if the database is
    /// remote.
    ///
    pub async fn initialize() -> Result<Surreal<Db>, Error> {
        let mut endpoint = dirs::home_dir()
            .ok_or(Error::HomeDirNotFound)?
            .join(".pilum")
            .join(Self::DATABASE);

        if std::env::var("PILUM_MODE").is_ok_and(|m| m == "test") {
            endpoint = std::env::temp_dir()
                .join(uuid::Uuid::new_v4().to_string())
                .join(Self::DATABASE);
        }

        Self::connect(endpoint).await
    }

    /// Connects to the SurrealDB database at the specified endpoint. The endpoint
    /// is the path to the database file. The function returns a `Result` that
    /// contains the `SurrealDb` instance if the connection is successful, or a
    /// `SurrealError` if the connection fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the SurrealDB database connection
    /// fails. This could be due to a variety of reasons, such as the database file
    /// not existing, insufficient permissions or a network error if the database is
    /// remote.
    ///
    async fn connect(endpoint: PathBuf) -> Result<Surreal<Db>, Error> {
        let db = Surreal::new::<RocksDb>(endpoint).await?;
        db.use_ns(Self::NAMESPACE).use_db(Self::DATABASE).await?;
        Ok(db)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test checks the initialization of the SurrealDB database. It uses the
    // `initialize` method of the `Database` struct to attempt to connect to the
    // database. The `assert!` macro is then used to check that the `Result`
    // returned by the `initialize` method is `Ok`.
    #[tokio::test]
    async fn test_database_initialization() {
        std::env::set_var("PILUM_MODE", "test");
        let db = Database::initialize().await;
        assert!(db.is_ok(), "Database initialization failed.");
    }
}
