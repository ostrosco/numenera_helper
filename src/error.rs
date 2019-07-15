#[derive(Debug)]
pub enum NumeneraError {
    DatabaseError,
    DataFormatError,
}

impl std::fmt::Display for NumeneraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumeneraError::DatabaseError => write!(f, "Database Error"),
            NumeneraError::DataFormatError => write!(f, "Data Format Error"),
        }
    }
}

impl std::error::Error for NumeneraError {}

impl From<rusqlite::Error> for NumeneraError {
    fn from(_: rusqlite::Error) -> NumeneraError {
        NumeneraError::DatabaseError
    }
}

impl From<std::num::ParseIntError> for NumeneraError {
    fn from(_: std::num::ParseIntError) -> NumeneraError {
        NumeneraError::DataFormatError
    }
}

impl From<regex::Error> for NumeneraError {
    fn from(_: regex::Error) -> NumeneraError {
        NumeneraError::DataFormatError
    }
}
