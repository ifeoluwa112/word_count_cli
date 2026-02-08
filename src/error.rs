use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Args(String),
    Io(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Args(msg) => write!(f, "Argument error: {}", msg),
            AppError::Io(err) => write!(f, "File error: {}", err),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}
