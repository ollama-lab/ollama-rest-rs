use serde::{Deserialize, Serialize};
use serde_json::Map;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingGenerationRequest {
    pub model: String,
    pub prompt: String,

    pub options: Option<Map<String, serde_json::Value>>,
    pub keep_alive: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingGenerationResponse {
    pub embedding: Vec<f64>,
}
