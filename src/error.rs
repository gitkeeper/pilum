#[derive(Debug)]
/// TODO: Add comment
pub enum Error {
    DatabaseError(surrealdb::Error),
    HomeDirNotFound,
    TempDirCreationFailed,
}

/// TODO: Add comment
impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Error {
        Error::DatabaseError(error)
    }
}
