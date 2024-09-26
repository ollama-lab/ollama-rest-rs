//! Completion streaming using Stream API
//!
//! In this example, you should see a streamed text output from your local LLM,
//! just like you saw on ChatGPT =)

use std::io::Write;

use futures::StreamExt;
use ollama_rest::{models::generate::GenerationRequest, Ollama};
use serde_json::json;

#[tokio::main]
async fn main() {
    // Make sure Ollama serves at 127.0.0.1:11434
    let ollama = Ollama::default();

    let request = serde_json::from_value::<GenerationRequest>(json!({
        "model": "llama3.2:1b",
        "prompt": "Why is the sky blue?",
    })).unwrap();

    let mut stream = ollama.generate_streamed(&request).await.unwrap();

    while let Some(Ok(res)) = stream.next().await {
        if !res.done {
            print!("{}", res.response);
            // Flush stdout for each word to allow realtime output
            std::io::stdout().flush().unwrap();
        }
    }

    println!();
}
