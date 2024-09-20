use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum WateryError {
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,

    #[error("we dont have such vendor")]
    NoSuchVendor,

}

pub type WateryResult<T> = Result<T, WateryError>;

