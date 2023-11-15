use http::StatusCode;
use thiserror::Error;

/// Error represents the error type of the library.
#[derive(Debug, Error)]
pub enum Error {
    #[error("status:{0}, message:{1}")]
    ApiError(StatusCode, String),

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::ApiError(status_code1, error_message1),
                Self::ApiError(status_code2, error_message2),
            ) => status_code1 == status_code2 && error_message1 == error_message2,
            (Self::RequestError(err1), Self::RequestError(err2)) => {
                err1.to_string() == err2.to_string()
            }
            _ => false,
        }
    }
}

/// Result alias where the error type is [`crate::Error`].
pub type Result<T> = std::result::Result<T, Error>;
