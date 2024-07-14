use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreationRequest {
    pub name: String,
    pub modelfile: Option<String>,
    pub stream: Option<bool>,
    pub path: Option<PathBuf>,
}
