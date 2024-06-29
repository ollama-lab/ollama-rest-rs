use std::collections::HashMap;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelDetails {
    pub parent_model: Option<String>,
    pub format: String,
    pub family: String,
    pub families: Option<Vec<String>>,
    pub parameter_size: String,
    pub quantization_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub modified_at: DateTime<Local>,
    pub size: usize,
    pub digest: String,
    pub details: ModelDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelListResponse {
    pub models: Vec<Model>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelShowRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelShowResponse {
    pub modelfile: String,
    pub parameters: String,
    pub template: String,

    pub details: ModelDetails,

    pub model_info: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelCopyRequest {
    pub source: String,
    pub destination: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelDeletionRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelSyncRequest {
    pub name: String,
    pub insecure: Option<bool>,
    pub stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelStatus {
    pub status: String,
    pub digest: Option<String>,
    pub total: Option<usize>,
    pub completed: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunningModelResponse {
    pub name: String,
    pub model: String,
    pub size: usize,
    pub digest: String,

    pub details: ModelDetails,

    pub expires_at: DateTime<Local>,
    pub size_vram: usize,
}
