use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AppError {
    NotFound,
    AlreadyExists(u64),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl error::Error for AppError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
