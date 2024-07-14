use std::io::Write;

use ollama_rest::{prelude::*, Ollama};

// Use qwen2:0.5b because it is good for demonstration due to its size.
const MODEL_NAME: &str = "qwen2:0.5b";

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();

    let mut prev_status = String::new();

    ollama.pull_model(&serde_json::from_value::<ModelSyncRequest>(serde_json::json!({
        "name": MODEL_NAME,
    })).unwrap(), Some(|res: &ModelPullStatus| {
        if &res.status[..] != prev_status.as_str() {
            prev_status = res.status.clone();
            println!("\n{}", res.status);
        }

        if let Some(progress) = &res.download_info {
            print!("\r{} / {}", progress.completed.unwrap_or(0), progress.total);
            std::io::stdout().flush().unwrap();
        }
    })).await.unwrap();
}
