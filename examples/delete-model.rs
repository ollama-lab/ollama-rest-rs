use std::io::Write;

use ollama_rest::{prelude::*, Ollama};

// Use llama3.2:1b because it is good for demonstration due to its size.
const MODEL_NAME: &str = "llama3.2:1b";

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();

    print!("Deleting {MODEL_NAME}... ");
    std::io::stdout().flush().unwrap();

    ollama.delete_model(&serde_json::from_value::<ModelDeletionRequest>(serde_json::json!({
        "name": MODEL_NAME,
    })).unwrap()).await.unwrap();

    println!("done");
}
