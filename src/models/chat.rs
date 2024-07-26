use std::{collections::BTreeMap, fmt::Display, str::FromStr};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::{errors::ParsingError, json_schema::DataStructure, RequestFormat};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::System => "system",
            Self::User => "user",
            Self::Assistant => "assistant",
        })
    }
}

impl FromStr for Role {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "system" => Role::System,
            "user" => Role::User,
            "assistant" => Role::Assistant,
            _ => Err(ParsingError::InvalidStr)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolCall {
    Function {
        name: String,
        arguments: BTreeMap<String, String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub images: Option<Vec<String>>,
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
    pub tools: Option<Vec<DataStructure>>,
}

/// Chat completion response
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub model: String,
    pub created_at: DateTime<Local>,
    pub message: Option<Message>,
    pub done: bool,

    pub total_duration: Option<u64>,
    pub load_duration: Option<u64>,
    pub prompt_eval_count: Option<usize>,
    pub prompt_eval_duration: Option<u64>,
    pub eval_count: Option<usize>,
    pub eval_duration: Option<u64>,
}
