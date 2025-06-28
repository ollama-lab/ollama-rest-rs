use std::{collections::BTreeMap, fmt::Display, str::FromStr};

#[cfg(feature = "chrono")]
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::{errors::ParsingError, json_schema::JsonSchema, RequestFormat};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

impl Role {
    pub fn as_str(&self) -> &str {
        match self {
            Self::System => "system",
            Self::User => "user",
            Self::Assistant => "assistant",
            Self::Tool => "tool",
        }
    }
}

impl AsRef<str> for Role {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Role {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "system" => Role::System,
            "user" => Role::User,
            "assistant" => Role::Assistant,
            "tool" => Role::Tool,
            _ => Err(ParsingError::InvalidStr)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolCall {
    Function {
        name: String,
        arguments: BTreeMap<String, Value>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub images: Option<Vec<String>>,
    /// Tool calls
    ///
    /// Since 0.3.0
    pub tool_calls: Option<Vec<ToolCall>>,
}

/// Chat completion request
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    
    pub format: Option<RequestFormat>,
    pub options: Option<Map<String, Value>>,
    pub stream: Option<bool>,
    pub keep_alive: Option<String>,
    /// Tool definition
    ///
    /// Since 0.3.0
    pub tools: Option<Vec<JsonSchema>>,
}

/// Chat completion response
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub model: String,

    #[cfg(feature = "chrono")]
    pub created_at: DateTime<Local>,
    #[cfg(not(feature = "chrono"))]
    pub created_at: String,

    pub message: Option<Message>,
    pub done: bool,

    pub total_duration: Option<u64>,
    pub load_duration: Option<u64>,
    pub prompt_eval_count: Option<usize>,
    pub prompt_eval_duration: Option<u64>,
    pub eval_count: Option<usize>,
    pub eval_duration: Option<u64>,
}
