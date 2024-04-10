//! # Database
//!
//! This module provides the `Database` struct which is used to interact with the SurrealDB
//! database. This struct provides methods for initializing and interacting with the database.
//!
//! The `initialize` method is used to create a new instance of the `Database` struct and connect to
//! the SurrealDB database. It uses the `NAMESPACE` and `DATABASE` constants to specify the
//! namespace and database for SurrealDB to use.
//!
//! The `path` method is used to determine the path to the database file. It uses the `dirs` crate
//! to get the home directory and appends the `.pilum/database` directory to it.
//!
use dirs::home_dir;
use std::path::PathBuf;
use surrealdb::engine::local::RocksDb;
use surrealdb::Surreal;

type SurrealDb = Surreal<surrealdb::engine::local::Db>;
type SurrealError = surrealdb::Error;

pub struct Database;

impl Database {
    const NAMESPACE: &'static str = "pilum";
    const DATABASE: &'static str = "pilum";

    /// Initializes a new instance of the `Database` struct and connects to the SurrealDB database.
    ///
    /// This method creates a new `SurrealDb` instance and sets the namespace and database to use
    /// based on the `NAMESPACE` and `DATABASE` constants. It returns a `Result` that contains the
    /// `SurrealDb` instance if the database connection is successful, or a `SurrealError` if the
    /// connection fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the SurrealDB database connection fails. This could be
    /// due to a variety of reasons, such as the database file not existing, insufficient
    /// permissions or a network error if the database is remote.
    ///
    pub async fn initialize() -> Result<SurrealDb, SurrealError> {
        let db = Surreal::new::<RocksDb>(Self::endpoint()).await?;
        db.use_ns(Self::NAMESPACE).use_db(Self::DATABASE).await?;
        Ok(db)
    }

    /// Determines the endpoint to the database file.
    ///
    /// This method uses the `dirs` crate to get the home directory and appends the a database
    /// directory to it. It returns a `PathBuf` that represents the endpoint to the database file.
    ///
    /// # Errors
    ///
    /// This function will return an error if the home directory cannot be determined. This could be
    /// due to a variety of reasons, such as the HOME environment variable not being set.
    ///
    fn endpoint() -> PathBuf {
        home_dir().unwrap().join(".pilum").join("database")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test checks the initialization of the SurrealDB database. It uses the `initialize`
    // method of the `Database` struct to attempt to connect to the database. The `assert!` macro is
    // then used to check that the `Result` returned by the `initialize` method is `Ok`.
    #[tokio::test]
    async fn test_database_initialization() {
        let result = Database::initialize().await;
        assert!(result.is_ok(), "Database initialization failed.");
    }

    // This test checks the `endpoint` function of the `Database` struct. It calls the `endpoint`
    // function and asserts that the returned `PathBuf` are correct.
    #[test]
    fn test_endpoint_function() {
        let endpoint = Database::endpoint();
        assert!(endpoint.is_absolute(), "Path to endpoint is not absolute.");
        assert_eq!(
            endpoint,
            home_dir().unwrap().join(".pilum").join("database"),
            "Path to endpoint is incorrect."
        );
    }
}
