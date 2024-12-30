use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AppError {
    message: String,
}

impl AppError {
    pub fn new(msg: impl Into<String>) -> Self {
        AppError {
            message: msg.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AppError {}

impl From<Box<dyn Error>> for AppError {
    fn from(err: Box<dyn Error>) -> Self {
        AppError::new(err.to_string())
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::new(err)
    }
}

impl warp::reject::Reject for AppError {}
