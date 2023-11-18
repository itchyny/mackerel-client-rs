use derivative::Derivative;
use http::StatusCode;
use thiserror::Error;

/// Error represents the error type of the library.
#[derive(Debug, Derivative, Error)]
#[derivative(PartialEq)]
pub enum Error {
    #[error("status_code:{0}, message:{1}")]
    ApiError(StatusCode, String),

    #[error(transparent)]
    RequestError(
        #[from]
        #[derivative(PartialEq = "ignore")]
        reqwest::Error,
    ),
}

/// Result alias where the error type is [`crate::Error`].
pub type Result<T> = std::result::Result<T, Error>;
