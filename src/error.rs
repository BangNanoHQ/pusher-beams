use reqwest::{header::InvalidHeaderValue, StatusCode};



#[derive(thiserror::Error, Debug)]
pub enum Error {
    // #[error("Repo MainDB error: `{0}`")]
    // MainDBError(std::Error),

    #[error("Request Error: {0}")]
    RequestError(reqwest::Error),

    #[error("Deserialization Error: {0}")]
    DeserializationError(serde_json::Error),

    #[error("Invalid Header value Error: {0}")]
    InvalidHeaderValueError(InvalidHeaderValue),

    #[error("Not allowed: {0}")]
    NotAllowed(String),

    #[error("Response Error: {0}")]
    ResponseError(String),

    #[error("Other Error: {0}")]
    OtherError(String),

    #[error("unknown repo error")]
    Unknown,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::RequestError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::DeserializationError(e)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(e: InvalidHeaderValue) -> Self {
        Error::InvalidHeaderValueError(e)
    }
}