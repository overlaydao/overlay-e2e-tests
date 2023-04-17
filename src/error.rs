use anyhow::anyhow;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Invalid argument: {cause}")]
    InvalidArgumentError { cause: String },

    #[error("Node error: {cause}")]
    NodeError { cause: anyhow::Error },

    #[error("System error: {cause}")]
    SystemError { cause: anyhow::Error },
}

impl Error {
    pub fn new_invalid_argument_error<S: std::fmt::Display>(cause: S) -> Self {
        Self::InvalidArgumentError {
            cause: cause.to_string(),
        }
    }
}

impl From<concordium_rust_sdk::endpoints::Error> for Error {
    fn from(value: concordium_rust_sdk::endpoints::Error) -> Self {
        Self::NodeError {
            cause: value.into(),
        }
    }
}

impl From<concordium_rust_sdk::endpoints::RPCError> for Error {
    fn from(value: concordium_rust_sdk::endpoints::RPCError) -> Self {
        Self::NodeError {
            cause: value.into(),
        }
    }
}

impl From<concordium_rust_sdk::endpoints::QueryError> for Error {
    fn from(value: concordium_rust_sdk::endpoints::QueryError) -> Self {
        Self::NodeError {
            cause: value.into(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::SystemError {
            cause: anyhow!(value),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SystemError {
            cause: anyhow!(value),
        }
    }
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self::SystemError { cause: value }
    }
}
