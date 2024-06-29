use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingGenerationRequest {
    pub model: String,
    pub prompt: String,

    pub options: Option<HashMap<String, serde_json::Value>>,
    pub keep_alive: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingGenerationResponse {
    pub embedding: Vec<f64>,
}
