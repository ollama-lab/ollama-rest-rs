use ollama_rest::{models::embeddings::EmbeddingGenerationRequest, Ollama};

#[tokio::main]
async fn main() {
    // Make sure Ollama serves at 127.0.0.1:11434
    let ollama = Ollama::default();

    let res = ollama.generate_embeddings(
        // Or use `serde_json::json` macro to convert JSON to the model
        &EmbeddingGenerationRequest {
            model: "llama3".to_string(),
            prompt: "How are you today?".to_string(),
            options: None,
            keep_alive: None,
        }
    ).await.unwrap();

    println!("Embeddings");
    println!("=========");

    let mut to_display_remained: i32 = 10;

    for embedding in res.embedding.iter() {
        to_display_remained -= 1;

        if to_display_remained < 0 {
            break;
        }

        println!("{embedding}");
    }

    if to_display_remained < 0 {
        println!("...");
    }
}
