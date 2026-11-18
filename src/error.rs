use http::StatusCode;
use thiserror::Error;

/// Error represents the error type of the library.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("status_code:{0}, message:{1}")]
    ApiError(StatusCode, String),

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error("failed to decode response: {0}")]
    DecodeError(#[from] serde_json::Error),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Error::ApiError(status1, message1), Error::ApiError(status2, message2))
                if status1 == status2 && message1 == message2
        )
    }
}

/// Result alias where the error type is [`crate::Error`].
pub type Result<T> = std::result::Result<T, Error>;
