use futures::StreamExt;
use ollama_rest::{models::model::ModelSyncRequest, Ollama};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut model_name: Option<String> = None;
    let mut confirmed = false;

    for arg in args.iter().skip(1) {
        if arg.starts_with("-") {
            confirmed = confirmed || arg.as_str() == "--confirm";
        } else if model_name.is_none() {
            model_name = Some(arg.to_string());
        }
    }

    if model_name.is_none() {
        println!("Error: No model name provided");
        return;
    }

    if !confirmed {
        print_warning_text();
        return;
    }

    // Make sure Ollama serves at 127.0.0.1:11434
    let ollama = Ollama::default();

    // Stream API
    //
    // For Callback API, see `Ollama::push_model()`.
    let mut stream = ollama.push_model_streamed(
        // Or using serde_json::json macro to convert JSON into the model
        &ModelSyncRequest {
            name: model_name.unwrap(),
            stream: None,
            insecure: None,
        }
    ).await.unwrap();

    let mut prev_status = String::new();

    while let Some(Ok(res)) = stream.next().await {
        if !prev_status.starts_with(res.status.as_str()) {
            prev_status = res.status.clone();
            println!("\n{}", res.status);
        }
    }
}

fn print_warning_text() {
    println!(r#"
#########################
#        WARNING        #
#########################

Example aborted to prevent users from spamming ollama.com with garbage models.

If you know what you are doing, append `--confirm` to the arguments to proceed.
"#);
}
