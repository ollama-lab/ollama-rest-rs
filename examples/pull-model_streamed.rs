use std::io::Write;

use futures::StreamExt;
use ollama_rest::{prelude::*, Ollama};

// Use qwen2:0.5b because it is good for demonstration due to its size.
const MODEL_NAME: &str = "qwen2:0.5b";

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();

    let mut prev_status = String::new();

    let mut stream = ollama.pull_model_streamed(
        &serde_json::from_value::<ModelSyncRequest>(serde_json::json!({
            "name": MODEL_NAME,
        }),).unwrap()
    ).await.unwrap();

    while let Some(Ok(res)) = stream.next().await {
        if !prev_status.starts_with(res.status.as_str()) {
            prev_status = res.status.clone();
            println!("\n{}", res.status);
        }

        if let Some(progress) = &res.download_info {
            print!("\r{} / {}", progress.completed.unwrap_or(0), progress.total);
            std::io::stdout().flush().unwrap();
        }
    }
}
