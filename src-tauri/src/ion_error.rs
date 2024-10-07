use std::io;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum IonError {
    #[error("Unknown data store error")]
    Unknown,

    #[error("No such provider")]
    NoSuchProvider,

    #[error("Connection to auth server failed")]
    AuthConnectionFailed,

    #[error("Serialization failed: {0}")]
    SerdeFailed(String),

    #[error("File IO error: {0}")]
    FileIOError(String),

    #[error("Connection to local server failed: {0}")]
    LocalServerConnectionError(String),

    #[error("State not found")]
    StateNotFound,
}

impl From<serde_json::Error> for IonError {
    fn from(err: serde_json::Error) -> IonError {
        IonError::SerdeFailed(err.to_string())
    }
}

impl From<io::Error> for IonError {
    fn from(err: io::Error) -> IonError {
        IonError::FileIOError(err.to_string())
    }
}

pub type IonResult<T> = Result<T, IonError>;
