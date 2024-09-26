use ollama_rest::{models::model::ModelCopyRequest, Ollama};

#[tokio::main]
async fn main() {
    // Make sure Ollama serves at 127.0.0.1:11434
    let ollama = Ollama::default();

    ollama.copy_model(
        &ModelCopyRequest {
            source: "llama3.2:1b".to_string(),
            destination: "rest-test-foobar".to_string(),
        },
    ).await.unwrap();

    println!("done");
}
