//! Serde models

use std::{fmt::Display, str::FromStr};

use errors::ParsingError;
use serde::{Deserialize, Serialize};

pub mod chat;
pub mod create;
pub mod embeddings;
pub mod errors;
pub mod generate;
pub mod model;

/// Request format
#[derive(Debug, Serialize, Deserialize)]
pub enum RequestFormat {
    /// JSON
    ///
    /// Currently the one and only format support by Ollama
    #[serde(rename = "json")]
    Json,
}

impl Display for RequestFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Json => "json",
        })
    }
}

impl FromStr for RequestFormat {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "json" => Self::Json,
            _ => Err(ParsingError::InvalidStr)?,
        })
    }
}

/// Status message
#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub status: String,
}
