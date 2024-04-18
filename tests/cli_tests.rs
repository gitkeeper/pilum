//! # CLI Tests
//!
//! This is a test operation for the application's command-line interface (CLI).
//!
//! `trycmd::TestCases::new()` creates a new set of test cases. Each `.case()`
//! method appends a new test case. The paths in the arguments represent test
//! files which contain the definition of each test case. The library `trycmd`
//! reads these `.md` files and the CLI tests will appraise different sets of
//! commands based on these definitions.
//!
use pilum::database::Database;

#[tokio::test]
async fn commands() {
    Database::cleanup_test().expect("Testing cleanup failed.");

    trycmd::TestCases::new()
        .env("PILUM_MODE", "test")
        .case("tests/commands/*.md");
}
