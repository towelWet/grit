use super::GritError;

impl From<&str> for GritError {
    fn from(value: &str) -> Self {
        GritError::Internal(value.into())
    }
}

impl From<walkdir::Error> for GritError {
    fn from(error: walkdir::Error) -> Self {
        GritError::Io(std::io::Error::other(error.to_string()))
    }
}
