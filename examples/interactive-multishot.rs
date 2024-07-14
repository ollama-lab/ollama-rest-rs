//! Your LLM might mention the "previous talk" in this chat!

use ollama_rest::{models::chat::ChatRequest, Ollama};
use serde_json::json;

#[tokio::main]
async fn main() {
    // Make sure Ollama serves at 127.0.0.1:11434
    let ollama = Ollama::default();

    let request = serde_json::from_value::<ChatRequest>(json!({
    }));
}
