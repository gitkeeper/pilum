//! # Main
//!
//! This is the main entry point for the application.
//!
//! The `main.rs` file is responsible for starting the application and
//! initializing the Tokio runtime environment. The Tokio runtime is used to
//! handle asynchronous tasks throughout the application.
//!
//! The `main` function uses the `#[tokio::main]` attribute to start the Tokio
//! runtime and then calls the `run` function of the `Cli` struct from the
//! crate module. The `run` function handles the parsing of command-line
//! arguments and the execution of the corresponding commands.
//!
//! The application uses the SurrealDB database for data persistence. The
//! database is initialized in the `run` function of the `Cli` struct.
//!
#[tokio::main]
async fn main() -> Result<(), pilum::Error> {
    pilum::Cli::run().await?;
    Ok(())
}
