use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::RequestFormat;

/// One-shot completion JSON request
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub model: String,
    pub prompt: Option<String>,
    pub images: Option<Vec<String>>,

    pub format: Option<RequestFormat>,

    pub options: Option<Map<String, Value>>,
    pub system: Option<String>,
    pub template: Option<String>,

    pub stream: Option<bool>,
    pub raw: Option<bool>,
    pub keep_alive: Option<String>,
}

/// One-shot completion JSON response
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationResponse {
    pub model: String,
    pub created_at: DateTime<Local>,
    pub response: String,
    pub done: bool,
    
    pub total_duration: Option<u64>,
    pub load_duration: Option<u64>,
    pub prompt_eval_count: Option<usize>,
    pub prompt_eval_duration: Option<u64>,
    pub eval_count: Option<usize>,
    pub eval_duration: Option<u64>,

    pub context: Option<Vec<u32>>,
}
