use http::StatusCode;
use std::time::Duration;
use thiserror::Error;

/// Error represents the error type of the library.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("status_code:{status_code}, message:{message}")]
    ApiError {
        status_code: StatusCode,
        message: String,
        retry_after: Option<Duration>,
    },

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error("failed to decode response: {0}")]
    DecodeError(#[from] serde_json::Error),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Error::ApiError {
                    status_code: status_code1,
                    message: message1,
                    retry_after: retry_after1,
                },
                Error::ApiError {
                    status_code: status_code2,
                    message: message2,
                    retry_after: retry_after2,
                },
            ) => {
                status_code1 == status_code2 && message1 == message2 && retry_after1 == retry_after2
            }
            _ => false,
        }
    }
}

/// Result alias where the error type is [`crate::Error`].
pub type Result<T> = std::result::Result<T, Error>;
