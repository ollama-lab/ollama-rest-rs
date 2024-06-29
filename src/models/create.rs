use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub modelfile: Option<String>,
    pub stream: Option<bool>,
    pub path: Option<PathBuf>,
}
