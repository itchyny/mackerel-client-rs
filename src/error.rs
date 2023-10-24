/// Error type
#[derive(Debug)]
pub enum Error {
    ApiError(reqwest::StatusCode, String),
    MsgError(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::MsgError(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::MsgError(s.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ApiError(status, message) => {
                write!(f, "status:{}, message:{}", status, message)
            }
            Error::MsgError(message) => write!(f, "message:{}", message),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
