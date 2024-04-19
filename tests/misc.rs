//! # Misc
//!
//! These are miscellaneous integration tests for the application's command-line
//! interface (CLI).
//!
//! `trycmd::TestCases::new()` creates a new set of test cases. Each `.case()`
//! method appends a new test case. The paths in the arguments represent test
//! files which contain the definition of each test case. The library `trycmd`
//! reads these `.md` files and evaluates the results for their correctness.
//!
#[test]
fn use_case_help() {
    trycmd::TestCases::new()
        .env("PILUM_MODE", "test")
        .case("tests/misc/*.md");
}
