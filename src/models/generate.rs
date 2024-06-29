use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use super::RequestFormat;

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    pub images: Option<Vec<String>>,

    pub format: Option<RequestFormat>,
    // TODO: Modelfile support
    pub stream: Option<bool>,
    pub raw: Option<bool>,
    pub keep_alive: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateResponse {
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
