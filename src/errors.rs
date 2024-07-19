//! Error module

use std::fmt::Display;

use reqwest::StatusCode;

#[derive(Debug)]
pub enum Error {
    ClientCreation(reqwest::Error),
    EmptyResponse,
    ErrorStatus(StatusCode),
    Event,
    NoCallback,
    NotExists,
    StreamingOff,
    UrlParsing(url::ParseError),
    JsonDecoding(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ClientCreation(value)
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParsing(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonDecoding(value)
    }
}
