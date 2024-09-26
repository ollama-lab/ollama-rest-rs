//! Completion streaming using Callback API
//!
//! In this example, you should see a streamed text output from your local LLM,
//! just like you saw on ChatGPT =)

use std::io::Write;

use ollama_rest::{models::generate::{GenerationRequest, GenerationResponse}, Ollama};
use serde_json::json;

#[tokio::main]
async fn main() {
    // Make sure Ollama serves at 127.0.0.1:11434
    let ollama = Ollama::default();

    let request = serde_json::from_value::<GenerationRequest>(json!({
        "model": "llama3.2:1b",
        "prompt": "Why is the sky blue?",
    })).unwrap();

    let final_res = ollama.generate(
        &request,
        Some(|res: &GenerationResponse| {
            if !res.done {
                print!("{}", res.response);
                // Flush stdout for each word to allow realtime output
                std::io::stdout().flush().unwrap();
            }
        })
    ).await.unwrap();

    println!("\n\nFinal response:");
    println!("{final_res:?}");
}
