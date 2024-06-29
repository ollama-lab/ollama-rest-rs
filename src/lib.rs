pub mod models;

/// The Ollama object that encapsulates everything you need.
pub struct Ollama {
    host: String,
}

impl Ollama {
    #[must_use]
    pub fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
        }
    }

    /// Get host info
    pub fn host(&self) -> &str {
        self.host.as_str()
    }
}

impl Default for Ollama {
    fn default() -> Self {
        Self::new("127.0.0.1:11434")
    }
}
