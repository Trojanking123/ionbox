use std::io;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum IonError {
    #[error("unknown data store error")]
    Unknown,

    #[error("we dont have such provider")]
    NoSuchProvider,

    #[error("connection to auth server failed")]
    AuthConnectionFailed,

    #[error("try to serde failed: {0}")]
    SerdeFailed(String),

    #[error("file io error: {0}")]
    FileIOError(String),
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
