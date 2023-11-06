use http::StatusCode;

/// Error type
#[derive(PartialEq, Debug)]
pub enum Error {
    ApiError(StatusCode, String),
    MsgError(String),
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
