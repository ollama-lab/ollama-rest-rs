use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::Map;

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
    pub verbose: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelShowResponse {
    pub modelfile: Option<String>,
    pub parameters: Option<String>,
    pub template: Option<String>,

    pub details: Option<ModelDetails>,

    pub model_info: Option<Map<String, serde_json::Value>>,
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
pub struct ModelDownloadStatus {
    pub digest: String,
    pub total: usize,
    pub completed: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelPullStatus {
    pub status: String,
    #[serde(flatten)]
    pub download_info: Option<ModelDownloadStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelUploadStatus {
    pub digest: String,
    pub total: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelPushStatus {
    pub status: String,
    #[serde(flatten)]
    pub upload_info: Option<ModelUploadStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunningModel {
    pub name: String,
    pub model: String,
    pub size: usize,
    pub digest: String,

    pub details: ModelDetails,

    pub expires_at: DateTime<Local>,
    pub size_vram: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunningModelResponse {
    pub models: Vec<RunningModel>,
}
